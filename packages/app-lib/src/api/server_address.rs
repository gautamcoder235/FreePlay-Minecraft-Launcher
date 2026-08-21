use crate::{Error, ErrorKind, Result};
use std::fmt::Display;
use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr};
use tokio::sync::Semaphore;

#[derive(Debug, Clone)]
pub enum ServerAddress {
    Unresolved(String),
    Resolved {
        original_host: String,
        original_port: u16,
        resolved_host: String,
        resolved_port: u16,
    },
}

impl ServerAddress {
    pub async fn resolve(&mut self) -> Result<()> {
        match self {
            Self::Unresolved(address) => {
                let (host, port) = parse_server_address(address)?;
                let (resolved_host, resolved_port) =
                    resolve_server_address(host, port).await?;
                *self = Self::Resolved {
                    original_host: if host.len() == address.len() {
                        mem::take(address)
                    } else {
                        host.to_owned()
                    },
                    original_port: port,
                    resolved_host,
                    resolved_port,
                }
            }
            Self::Resolved { .. } => {}
        }
        Ok(())
    }

    pub fn require_resolved(&self) -> Result<(&str, u16)> {
        match self {
            Self::Resolved {
                resolved_host,
                resolved_port,
                ..
            } => Ok((resolved_host, *resolved_port)),
            Self::Unresolved(address) => Err(ErrorKind::InputError(format!(
                "Unexpected unresolved server address: {address}"
            ))
            .into()),
        }
    }
}

impl Display for ServerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unresolved(address) => write!(f, "{address}"),
            Self::Resolved {
                resolved_host,
                resolved_port,
                ..
            } => {
                if resolved_host.contains(':') {
                    write!(f, "[{resolved_host}]:{resolved_port}")
                } else {
                    write!(f, "{resolved_host}:{resolved_port}")
                }
            }
        }
    }
}

pub fn parse_server_address(address: &str) -> Result<(&str, u16)> {
    parse_server_address_inner(address)
        .map_err(|e| Error::from(ErrorKind::InputError(e)))
}

// Reimplementation of Guava's HostAndPort#fromString with a default port of 25565
fn parse_server_address_inner(
    address: &str,
) -> std::result::Result<(&str, u16), String> {
    let (host, port_str) = if address.starts_with("[") {
        let (Some(colon_index), Some(close_bracket_index)) =
            (address.find(':'), address.rfind(']'))
        else {
            return Err(format!("Invalid bracketed host/port: {address}"));
        };
        if close_bracket_index <= colon_index {
            return Err(format!("Invalid bracketed host/port: {address}"));
        }

        let host = &address[1..close_bracket_index];
        if close_bracket_index + 1 == address.len() {
            (host, "")
        } else {
            if address.as_bytes().get(close_bracket_index + 1).copied()
                != Some(b':')
            {
                return Err(format!(
                    "Only a colon may follow a close bracket: {address}"
                ));
            }
            let port_str = &address[close_bracket_index + 2..];
            for c in port_str.chars() {
                if !c.is_ascii_digit() {
                    return Err(format!("Port must be numeric: {address}"));
                }
            }
            (host, port_str)
        }
    } else {
        if let Some((host, port)) = address.split_once(':')
            && !port.contains(':')
        {
            (host, port)
        } else {
            (address, "")
        }
    };

    let mut port = None;
    if !port_str.is_empty() {
        if port_str.starts_with('+') {
            return Err(format!("Unparsable port number: {port_str}"));
        }
        port = port_str.parse::<u16>().ok();
        if port.is_none() {
            return Err(format!("Unparsable port number: {port_str}"));
        }
    }

    Ok((host, port.unwrap_or(25565)))
}

pub async fn resolve_server_address(
    host: &str,
    port: u16,
) -> Result<(String, u16)> {
    static SIMULTANEOUS_DNS_QUERIES: Semaphore = Semaphore::const_new(24);

    if port != 25565
        || host.parse::<Ipv4Addr>().is_ok()
        || host.parse::<Ipv6Addr>().is_ok()
    {
        return Ok((host.to_owned(), port));
    }

    let _permit = SIMULTANEOUS_DNS_QUERIES.acquire().await?;
    let resolver = hickory_resolver::TokioResolver::builder_tokio()?.build();
    Ok(
        match resolver.srv_lookup(format!("_minecraft._tcp.{host}")).await {
            Err(e)
                if e.proto()
                    .as_ref()
                    .is_some_and(|x| x.kind().is_no_records_found()) =>
            {
                None
            }
            Err(e) => return Err(e.into()),
            Ok(lookup) => lookup
                .into_iter()
                .next()
                .map(|r| (r.target().to_string(), r.port())),
        }
        .unwrap_or_else(|| (host.to_owned(), port)),
    )
}

// =========================================================================
// Server Hosting & Tunnel Tauri Commands
// =========================================================================

pub use crate::state::{HostStatus, ServerTelemetry};

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_start_server(
    version: String,
    server_type: String,
    ram_mb: u32,
    port: u16,
) -> Result<()> {
    let state = crate::State::get().await?;
    let working_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| {
            state
                .directories
                .config_dir
                .join("servers")
                .join(format!("{}-{}", server_type.to_lowercase(), version))
        });
    state
        .server_hosting
        .start_server(version, server_type, ram_mb, port, working_dir)
        .await
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_stop_server() -> Result<()> {
    let state = crate::State::get().await?;
    state.server_hosting.stop_server().await
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_send_command(command: String) -> Result<()> {
    let state = crate::State::get().await?;
    state.server_hosting.send_command(command).await
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_get_telemetry() -> Result<ServerTelemetry> {
    let state = crate::State::get().await?;
    Ok(state.server_hosting.get_telemetry().await)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_get_status() -> Result<HostStatus> {
    let state = crate::State::get().await?;
    Ok(state.server_hosting.get_status().await)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_start_tunnel(port: u16) -> Result<HostStatus> {
    let state = crate::State::get().await?;
    state.server_hosting.start_tunnel(port).await?;
    Ok(state.server_hosting.get_status().await)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_stop_tunnel() -> Result<HostStatus> {
    let state = crate::State::get().await?;
    state.server_hosting.stop_tunnel().await?;
    Ok(state.server_hosting.get_status().await)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_kill_server() -> Result<()> {
    let state = crate::State::get().await?;
    state.server_hosting.kill_server().await
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_open_server_dir() -> Result<()> {
    let state = crate::State::get().await?;
    let dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| {
            state
                .directories
                .config_dir
                .join("servers")
        });
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to create server directory {:?}: {e}",
                dir
            ))
        })?;
    }

    let dir_str = dir.to_string_lossy().to_string();
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("explorer")
                .arg(&dir_str)
                .spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open")
                .arg(&dir_str)
                .spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg(&dir_str)
                .spawn();
        }
    })
    .await
    .map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to open directory: {e}"
        ))
    })?;

    Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_update_config(
    server_id: Option<String>,
    version: Option<String>,
    engine: Option<String>,
    ram_gb: Option<u32>,
    motd: Option<String>,
    port: Option<u16>,
) -> Result<()> {
    let state = crate::State::get().await?;
    state
        .server_hosting
        .update_config(version.clone(), engine.clone(), ram_gb, motd, port)
        .await;

    let server_dir = if let Some(ref sid) = server_id {
        let p = std::path::PathBuf::from(sid);
        if p.is_absolute() && p.exists() {
            p
        } else {
            state.directories.config_dir.join("servers").join(sid)
        }
    } else if let Some(dir) = state.server_hosting.get_working_dir().await {
        dir
    } else {
        state.directories.config_dir.join("servers").join("default")
    };

    let _ = tokio::fs::create_dir_all(&server_dir).await;
    let meta_path = server_dir.join(".freeplay-server.json");
    let mut meta = if meta_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&meta_path).await {
            serde_json::from_str::<serde_json::Value>(&content)
                .unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        }
    } else {
        serde_json::json!({})
    };

    if let Some(v) = version {
        meta["version"] = serde_json::Value::String(v);
    }
    if let Some(e) = engine {
        meta["engine"] = serde_json::Value::String(e);
    }
    if let Some(r) = ram_gb {
        meta["ram_gb"] = serde_json::Value::Number(r.into());
    }
    if let Some(p) = port {
        meta["port"] = serde_json::Value::Number(p.into());
    }
    let _ = tokio::fs::write(
        &meta_path,
        serde_json::to_string_pretty(&meta).unwrap_or_default(),
    )
    .await;

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerFileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_read_file(path: String) -> Result<String> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let clean = path.trim_start_matches('/');
    let file_path = base_dir.join(clean);

    if !file_path.starts_with(&base_dir) {
        return Err(crate::ErrorKind::InputError(
            "Path traversal not allowed".to_string(),
        )
        .into());
    }

    let content = tokio::fs::read_to_string(&file_path).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to read file {:?}: {e}",
            file_path
        ))
    })?;
    Ok(content)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_save_file(path: String, content: String) -> Result<()> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let clean = path.trim_start_matches('/');
    let file_path = base_dir.join(clean);

    if !file_path.starts_with(&base_dir) {
        return Err(crate::ErrorKind::InputError(
            "Path traversal not allowed".to_string(),
        )
        .into());
    }

    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to create parent directory: {e}"
            ))
        })?;
    }

    tokio::fs::write(&file_path, content.as_bytes())
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to write file {:?}: {e}",
                file_path
            ))
        })?;
    Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_get_files(path: Option<String>) -> Result<Vec<ServerFileEntry>> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let target = if let Some(ref p) = path {
        let clean = p.trim_start_matches('/');
        base_dir.join(clean)
    } else {
        base_dir.clone()
    };

    if !target.starts_with(&base_dir) {
        return Err(crate::ErrorKind::InputError(
            "Path traversal not allowed".to_string(),
        )
        .into());
    }

    if !target.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&target).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to read directory {:?}: {e}",
            target
        ))
    })?;

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let metadata = entry.metadata().await.ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let name = entry.file_name().to_string_lossy().to_string();
        let rel_path = entry
            .path()
            .strip_prefix(&base_dir)
            .unwrap_or(entry.path().as_path())
            .to_string_lossy()
            .replace('\\', "/");

        entries.push(ServerFileEntry {
            name,
            path: format!("/{}", rel_path),
            is_dir,
            size,
        });
    }

    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
    });

    Ok(entries)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupEntry {
    pub id: String,
    pub name: String,
    pub date: String,
    pub size: String,
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_create_backup(name: String) -> Result<BackupEntry> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let world_dir = base_dir.join("world");
    let backups_dir = base_dir.join("backups");
    tokio::fs::create_dir_all(&backups_dir).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to create backups directory: {e}"
        ))
    })?;

    let backup_name = if name.ends_with(".zip") {
        name.clone()
    } else {
        format!("{}.zip", name)
    };
    let backup_path = backups_dir.join(&backup_name);

    if world_dir.exists() {
        let backup_path_clone = backup_path.clone();
        let world_dir_clone = world_dir.clone();
        tokio::task::spawn_blocking(move || {
            let file =
                std::fs::File::create(&backup_path_clone).map_err(|e| {
                    crate::ErrorKind::OtherError(format!(
                        "Failed to create backup file: {e}"
                    ))
                })?;
            let mut zip = zip::ZipWriter::new(file);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            fn add_dir_to_zip(
                zip: &mut zip::ZipWriter<std::fs::File>,
                dir: &std::path::Path,
                base: &std::path::Path,
                options: zip::write::SimpleFileOptions,
            ) -> std::io::Result<()> {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    let rel = path
                        .strip_prefix(base)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/");
                    if path.is_dir() {
                        zip.add_directory(format!("{}/", rel), options)?;
                        add_dir_to_zip(zip, &path, base, options)?;
                    } else {
                        zip.start_file(rel, options)?;
                        let data = std::fs::read(&path)?;
                        std::io::Write::write_all(zip, &data)?;
                    }
                }
                Ok(())
            }

            add_dir_to_zip(&mut zip, &world_dir_clone, &world_dir_clone, options)
                .map_err(|e| {
                    crate::ErrorKind::OtherError(format!(
                        "Failed to write backup zip: {e}"
                    ))
                })?;
            zip.finish().map_err(|e| {
                crate::ErrorKind::OtherError(format!(
                    "Failed to finalize backup zip: {e}"
                ))
            })?;
            Ok::<(), crate::Error>(())
        })
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!("Backup task failed: {e}"))
        })??;
    } else {
        return Err(crate::ErrorKind::InputError(
            "World directory does not exist yet — start the server first".to_string(),
        )
        .into());
    }

    let meta = tokio::fs::metadata(&backup_path).await.ok();
    let size = meta.map(|m| m.len()).unwrap_or(0);
    let size_str = if size > 1_048_576 {
        format!("{:.1} MB", size as f64 / 1_048_576.0)
    } else {
        format!("{:.1} KB", size as f64 / 1024.0)
    };

    Ok(BackupEntry {
        id: format!("backup-{}", chrono::Utc::now().timestamp_millis()),
        name: backup_name,
        date: chrono::Local::now().format("%B %d, %Y %H:%M").to_string(),
        size: size_str,
    })
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_restore_backup(backup_id: String) -> Result<()> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let backups_dir = base_dir.join("backups");
    let world_dir = base_dir.join("world");

    let backup_name = if backup_id.starts_with("backup-") {
        let mut found = None;
        if let Ok(mut rd) = tokio::fs::read_dir(&backups_dir).await {
            while let Ok(Some(e)) = rd.next_entry().await {
                found = Some(e.file_name().to_string_lossy().to_string());
            }
        }
        found.unwrap_or(backup_id)
    } else {
        backup_id
    };

    let backup_path = backups_dir.join(&backup_name);
    if !backup_path.exists() {
        return Err(crate::ErrorKind::InputError(format!(
            "Backup file not found: {}",
            backup_name
        ))
        .into());
    }

    if world_dir.exists() {
        tokio::fs::remove_dir_all(&world_dir).await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to remove existing world directory: {e}"
            ))
        })?;
    }
    tokio::fs::create_dir_all(&world_dir).await.ok();

    let world_dir_clone = world_dir.clone();
    let backup_path_clone = backup_path.clone();
    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&backup_path_clone).map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to open backup file: {e}"
            ))
        })?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to read backup zip: {e}"
            ))
        })?;
        archive.extract(&world_dir_clone).map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to extract backup: {e}"
            ))
        })?;
        Ok::<(), crate::Error>(())
    })
    .await
    .map_err(|e| {
        crate::ErrorKind::OtherError(format!("Restore task failed: {e}"))
    })??;

    Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_get_backups() -> Result<Vec<BackupEntry>> {
    let state = crate::State::get().await?;
    let base_dir = state
        .server_hosting
        .get_working_dir()
        .await
        .unwrap_or_else(|| state.directories.config_dir.join("servers"));

    let backups_dir = base_dir.join("backups");
    if !backups_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&backups_dir).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to read backups directory: {e}"
        ))
    })?;

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".zip") {
            continue;
        }
        let meta = entry.metadata().await.ok();
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = meta
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let dt: chrono::DateTime<chrono::Local> = t.into();
                dt.format("%B %d, %Y %H:%M").to_string()
            })
            .unwrap_or_default();
        let size_str = if size > 1_048_576 {
            format!("{:.1} MB", size as f64 / 1_048_576.0)
        } else {
            format!("{:.1} KB", size as f64 / 1024.0)
        };

        entries.push(BackupEntry {
            id: name.clone(),
            name,
            date: modified,
            size: size_str,
        });
    }

    Ok(entries)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub engine: String,
    pub version: String,
    pub port: u16,
    pub ram_gb: u32,
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_list_servers() -> Result<Vec<ServerEntry>> {
    let state = crate::State::get().await?;
    let servers_dir = state.directories.config_dir.join("servers");
    if !servers_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&servers_dir).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to read servers directory: {e}"
        ))
    })?;

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let meta_path = entry.path().join(".freeplay-server.json");
        if let Ok(content) = tokio::fs::read_to_string(&meta_path).await {
            if let Ok(meta) =
                serde_json::from_str::<serde_json::Value>(&content)
            {
                entries.push(ServerEntry {
                    id: name.clone(),
                    name: meta["name"]
                        .as_str()
                        .unwrap_or(&name)
                        .to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    engine: meta["engine"]
                        .as_str()
                        .unwrap_or("PaperMC")
                        .to_string(),
                    version: meta["version"]
                        .as_str()
                        .unwrap_or("1.21.4")
                        .to_string(),
                    port: meta["port"].as_u64().unwrap_or(25565) as u16,
                    ram_gb: meta["ram_gb"].as_u64().unwrap_or(4) as u32,
                });
                continue;
            }
        }
        entries.push(ServerEntry {
            id: name.clone(),
            name,
            path: entry.path().to_string_lossy().to_string(),
            engine: "PaperMC".to_string(),
            version: "1.21.4".to_string(),
            port: 25565,
            ram_gb: 4,
        });
    }

    Ok(entries)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_create_server(
    name: String,
    engine: String,
    version: String,
    port: u16,
    ram_gb: u32,
    custom_path: Option<String>,
) -> Result<ServerEntry> {
    let state = crate::State::get().await?;

    let server_dir = if let Some(ref cp) = custom_path {
        std::path::PathBuf::from(cp)
    } else {
        let safe_name = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>();
        state
            .directories
            .config_dir
            .join("servers")
            .join(&safe_name)
    };

    tokio::fs::create_dir_all(&server_dir).await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to create server directory: {e}"
        ))
    })?;

    let meta = serde_json::json!({
        "name": name,
        "engine": engine,
        "version": version,
        "port": port,
        "ram_gb": ram_gb,
    });
    let meta_path = server_dir.join(".freeplay-server.json");
    tokio::fs::write(&meta_path, serde_json::to_string_pretty(&meta).unwrap())
        .await
        .map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to write server metadata: {e}"
            ))
        })?;

    let id = server_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| name.clone());

    Ok(ServerEntry {
        id,
        name,
        path: server_dir.to_string_lossy().to_string(),
        engine,
        version,
        port,
        ram_gb,
    })
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_delete_server(server_id: String) -> Result<()> {
    let state = crate::State::get().await?;
    let server_dir = state
        .directories
        .config_dir
        .join("servers")
        .join(&server_id);

    if server_dir.exists() {
        tokio::fs::remove_dir_all(&server_dir).await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to delete server directory: {e}"
            ))
        })?;
    }
    Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_select_server(server_id: String) -> Result<()> {
    let state = crate::State::get().await?;

    let server_dir = std::path::PathBuf::from(&server_id);
    let dir = if server_dir.is_absolute() && server_dir.exists() {
        server_dir
    } else {
        state
            .directories
            .config_dir
            .join("servers")
            .join(&server_id)
    };

    if !dir.exists() {
        return Err(crate::ErrorKind::InputError(format!(
            "Server directory does not exist: {}",
            dir.display()
        ))
        .into());
    }

    state.server_hosting.set_working_dir(dir.clone()).await;

    let meta_path = dir.join(".freeplay-server.json");
    if let Ok(content) = tokio::fs::read_to_string(&meta_path).await {
        if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(v) = meta["version"].as_str() {
                *state.server_hosting.current_version.write().await =
                    Some(v.to_string());
            }
            if let Some(e) = meta["engine"].as_str() {
                *state.server_hosting.current_server_type.write().await =
                    Some(e.to_string());
            }
            if let Some(p) = meta["port"].as_u64() {
                *state.server_hosting.current_port.write().await = p as u16;
            }
            if let Some(r) = meta["ram_gb"].as_u64() {
                *state.server_hosting.current_ram_mb.write().await =
                    (r as u32) * 1024;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_server_address_inner;

    #[test]
    fn parses_ipv4_server_addresses() {
        for (address, expected) in [
            ("192.0.2.1", ("192.0.2.1", 25565)),
            ("192.0.2.1:25566", ("192.0.2.1", 25566)),
        ] {
            assert_eq!(parse_server_address_inner(address), Ok(expected));
        }
    }

    #[test]
    fn parses_ipv6_server_addresses() {
        for (address, expected) in [
            ("2001:db8::1", ("2001:db8::1", 25565)),
            ("[2001:db8::1]", ("2001:db8::1", 25565)),
            ("[2001:db8::1]:25566", ("2001:db8::1", 25566)),
        ] {
            assert_eq!(parse_server_address_inner(address), Ok(expected));
        }
    }
}

