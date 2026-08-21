use crate::{Error, ErrorKind, Result, State};
use futures::StreamExt;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerAddonEntry {
	pub id: String,
	pub name: String,
	pub source: String, // "modrinth" | "local"
	pub project_id: Option<String>,
	pub version_id: Option<String>,
	pub version_number: Option<String>,
	pub filename: String,
	pub addon_type: String, // "plugin" | "mod" | "datapack"
	pub game_version: Option<String>,
	pub file_size: u64,
	pub installed_at: String,
	pub enabled: bool,
	pub dependencies: Vec<String>,
	pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddonRegistry {
	pub addons: Vec<ServerAddonEntry>,
}

pub async fn get_target_server_dir(state: &State, server_id: Option<&str>) -> Result<PathBuf> {
	if let Some(id) = server_id {
		if !id.is_empty() {
			let custom = PathBuf::from(id);
			if custom.is_absolute() && custom.exists() {
				return Ok(custom);
			}
			let dir = state.directories.config_dir.join("servers").join(id);
			if dir.exists() {
				return Ok(dir);
			}
		}
	}

	if let Some(working_dir) = state.server_hosting.get_working_dir().await {
		if working_dir.exists() {
			return Ok(working_dir);
		}
	}

	let default_dir = state.directories.config_dir.join("servers");
	fs::create_dir_all(&default_dir).await.ok();
	Ok(default_dir)
}

pub async fn read_server_engine(server_dir: &Path) -> (String, String) {
	let meta_path = server_dir.join(".freeplay-server.json");
	if let Ok(content) = fs::read_to_string(&meta_path).await {
		if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&content) {
			let engine = meta["engine"].as_str().unwrap_or("PaperMC").to_string();
			let version = meta["version"].as_str().unwrap_or("1.21.4").to_string();
			return (engine, version);
		}
	}
	("PaperMC".to_string(), "1.21.4".to_string())
}

pub fn resolve_addon_dir(server_dir: &Path, engine: &str, addon_type: &str) -> PathBuf {
	let lower_engine = engine.to_lowercase();
	let lower_type = addon_type.to_lowercase();

	if lower_type == "datapack" {
		server_dir.join("world").join("datapacks")
	} else if lower_type == "mod" || lower_engine.contains("fabric") || lower_engine.contains("forge") || lower_engine.contains("neoforge") || lower_engine.contains("quilt") {
		server_dir.join("mods")
	} else {
		server_dir.join("plugins")
	}
}

pub async fn load_registry(server_dir: &Path) -> AddonRegistry {
	let registry_path = server_dir.join(".freeplay-addons.json");
	if let Ok(content) = fs::read_to_string(&registry_path).await {
		if let Ok(reg) = serde_json::from_str::<AddonRegistry>(&content) {
			return reg;
		}
	}
	AddonRegistry::default()
}

pub async fn save_registry(server_dir: &Path, registry: &AddonRegistry) -> Result<()> {
	let registry_path = server_dir.join(".freeplay-addons.json");
	let json = serde_json::to_string_pretty(registry).map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to serialize addon registry: {e}")))
	})?;
	fs::write(&registry_path, json).await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to write addon registry: {e}")))
	})?;
	Ok(())
}

fn is_valid_addon_file(name: &str) -> bool {
	name.ends_with(".jar") || name.ends_with(".jar.disabled") || name.ends_with(".zip") || name.ends_with(".zip.disabled")
}

pub async fn reconcile_registry(server_dir: &Path, engine: &str, mut registry: AddonRegistry) -> AddonRegistry {
	let folders = [
		resolve_addon_dir(server_dir, engine, "plugin"),
		resolve_addon_dir(server_dir, engine, "mod"),
		resolve_addon_dir(server_dir, engine, "datapack"),
	];

	// Remove entries whose files no longer exist anywhere in their target folders
	registry.addons.retain(|item| {
		for folder in &folders {
			let active_path = folder.join(&item.filename);
			let disabled_name = if item.filename.ends_with(".disabled") {
				item.filename.clone()
			} else {
				format!("{}.disabled", item.filename)
			};
			let disabled_path = folder.join(&disabled_name);
			if active_path.exists() || disabled_path.exists() {
				return true;
			}
		}
		false
	});

	// Discover new files on disk
	for folder in &folders {
		if !folder.exists() {
			continue;
		}
		if let Ok(mut rd) = fs::read_dir(folder).await {
			while let Ok(Some(entry)) = rd.next_entry().await {
				let file_name = entry.file_name().to_string_lossy().to_string();
				if !is_valid_addon_file(&file_name) {
					continue;
				}

				let is_disabled = file_name.ends_with(".disabled");
				let clean_name = if is_disabled {
					file_name.trim_end_matches(".disabled").to_string()
				} else {
					file_name.clone()
				};

				let already_tracked = registry.addons.iter_mut().find(|a| {
					a.filename == file_name || a.filename == clean_name || format!("{}.disabled", a.filename) == file_name
				});

				if let Some(existing) = already_tracked {
					existing.enabled = !is_disabled;
					existing.filename = file_name.clone();
					if let Ok(meta) = entry.metadata().await {
						existing.file_size = meta.len();
					}
				} else {
					let size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
					let inferred_type = if folder.ends_with("datapacks") {
						"datapack".to_string()
					} else if folder.ends_with("mods") {
						"mod".to_string()
					} else {
						"plugin".to_string()
					};

					let display_name = clean_name
						.trim_end_matches(".jar")
						.trim_end_matches(".zip")
						.replace('-', " ")
						.replace('_', " ");

					registry.addons.push(ServerAddonEntry {
						id: format!("local-{}", clean_name),
						name: display_name,
						source: "local".to_string(),
						project_id: None,
						version_id: None,
						version_number: None,
						filename: file_name,
						addon_type: inferred_type,
						game_version: None,
						file_size: size,
						installed_at: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
						enabled: !is_disabled,
						dependencies: Vec::new(),
						icon_url: None,
					});
				}
			}
		}
	}

	registry
}

fn validate_safe_filename(name: &str) -> Result<()> {
	if name.contains("..") || name.contains('/') || name.contains('\\') {
		return Err(ErrorKind::InputError("Invalid characters in filename".to_string()).into());
	}
	Ok(())
}

async fn atomic_download(url: &str, temp_path: &Path, final_path: &Path) -> Result<u64> {
	if let Some(parent) = temp_path.parent() {
		fs::create_dir_all(parent).await.ok();
	}
	if let Some(parent) = final_path.parent() {
		fs::create_dir_all(parent).await.ok();
	}

	let client = reqwest::Client::builder()
		.user_agent("FreePlay-Launcher/1.0")
		.build()
		.map_err(|e| Error::from(ErrorKind::OtherError(format!("Failed to build HTTP client: {e}"))))?;

	let response = client.get(url).send().await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to request download {url}: {e}")))
	})?;

	if !response.status().is_success() {
		return Err(ErrorKind::OtherError(format!(
			"Download failed with HTTP status: {}",
			response.status()
		))
		.into());
	}

	let mut file = fs::File::create(temp_path).await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to create temporary file {temp_path:?}: {e}")))
	})?;

	let mut stream = response.bytes_stream();
	let mut total_bytes: u64 = 0;

	while let Some(chunk_result) = stream.next().await {
		let chunk = chunk_result.map_err(|e| {
			Error::from(ErrorKind::OtherError(format!("Error streaming chunk from {url}: {e}")))
		})?;
		total_bytes += chunk.len() as u64;
		file.write_all(&chunk).await.map_err(|e| {
			Error::from(ErrorKind::OtherError(format!("Failed to write chunk to temporary file: {e}")))
		})?;
	}

	file.flush().await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to flush temporary file: {e}")))
	})?;
	drop(file);

	if total_bytes < 4 {
		fs::remove_file(temp_path).await.ok();
		return Err(ErrorKind::OtherError("Downloaded file is empty or too small to be a valid archive".to_string()).into());
	}

	// Validate magic bytes for ZIP/JAR (PK\x03\x04 or empty zip)
	if let Ok(mut f) = fs::File::open(temp_path).await {
		use tokio::io::AsyncReadExt;
		let mut magic = [0u8; 4];
		if f.read_exact(&mut magic).await.is_ok() && &magic != b"PK\x03\x04" && &magic[..2] != b"PK" {
			fs::remove_file(temp_path).await.ok();
			return Err(ErrorKind::OtherError("Downloaded file is not a valid JAR/ZIP archive".to_string()).into());
		}
	}

	// Atomic move/rename
	if final_path.exists() {
		fs::remove_file(final_path).await.ok();
	}
	fs::rename(temp_path, final_path).await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to move {temp_path:?} to {final_path:?}: {e}")))
	})?;

	Ok(total_bytes)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_list_installed_addons(server_id: Option<String>) -> Result<Vec<ServerAddonEntry>> {
	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let (engine, _) = read_server_engine(&server_dir).await;

	let registry = load_registry(&server_dir).await;
	let reconciled = reconcile_registry(&server_dir, &engine, registry).await;
	save_registry(&server_dir, &reconciled).await.ok();

	Ok(reconciled.addons)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_install_addon(
	server_id: Option<String>,
	name: String,
	project_id: Option<String>,
	version_id: Option<String>,
	version_number: Option<String>,
	filename: String,
	download_url: String,
	addon_type: String,
	game_version: Option<String>,
	dependencies: Option<Vec<String>>,
	icon_url: Option<String>,
) -> Result<ServerAddonEntry> {
	validate_safe_filename(&filename)?;
	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let (engine, _) = read_server_engine(&server_dir).await;

	let target_dir = resolve_addon_dir(&server_dir, &engine, &addon_type);
	fs::create_dir_all(&target_dir).await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to create addon directory {target_dir:?}: {e}")))
	})?;

	let downloads_dir = server_dir.join(".downloads");
	fs::create_dir_all(&downloads_dir).await.ok();

	let temp_part = downloads_dir.join(format!("{}.part", filename));
	let final_path = target_dir.join(&filename);

	let file_size = atomic_download(&download_url, &temp_part, &final_path).await?;

	let mut registry = load_registry(&server_dir).await;
	let entry_id = project_id.clone().unwrap_or_else(|| filename.clone());

	// Replace existing or append
	registry.addons.retain(|a| a.id != entry_id && a.filename != filename);

	let new_entry = ServerAddonEntry {
		id: entry_id,
		name,
		source: "modrinth".to_string(),
		project_id,
		version_id,
		version_number,
		filename,
		addon_type,
		game_version,
		file_size,
		installed_at: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
		enabled: true,
		dependencies: dependencies.unwrap_or_default(),
		icon_url,
	};

	registry.addons.push(new_entry.clone());
	save_registry(&server_dir, &registry).await.ok();

	Ok(new_entry)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_import_addon(
	server_id: Option<String>,
	file_path: String,
	addon_type: Option<String>,
) -> Result<ServerAddonEntry> {
	let src = PathBuf::from(&file_path);
	if !src.exists() || !src.is_file() {
		return Err(ErrorKind::InputError(format!("Source file does not exist: {file_path}")).into());
	}

	let file_name = src
		.file_name()
		.map(|n| n.to_string_lossy().to_string())
		.unwrap_or_else(|| "imported-addon.jar".to_string());

	validate_safe_filename(&file_name)?;

	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let (engine, _) = read_server_engine(&server_dir).await;

	let inferred_type = addon_type.unwrap_or_else(|| {
		if file_name.ends_with(".zip") {
			"datapack".to_string()
		} else if engine.to_lowercase().contains("fabric") || engine.to_lowercase().contains("forge") {
			"mod".to_string()
		} else {
			"plugin".to_string()
		}
	});

	let target_dir = resolve_addon_dir(&server_dir, &engine, &inferred_type);
	fs::create_dir_all(&target_dir).await.ok();

	let dest = target_dir.join(&file_name);
	fs::copy(&src, &dest).await.map_err(|e| {
		Error::from(ErrorKind::OtherError(format!("Failed to copy file to server {dest:?}: {e}")))
	})?;

	let size = fs::metadata(&dest).await.map(|m| m.len()).unwrap_or(0);
	let mut registry = load_registry(&server_dir).await;

	let clean_id = format!("local-{}", file_name);
	registry.addons.retain(|a| a.id != clean_id && a.filename != file_name);

	let display_name = file_name
		.trim_end_matches(".jar")
		.trim_end_matches(".zip")
		.replace('-', " ")
		.replace('_', " ");

	let new_entry = ServerAddonEntry {
		id: clean_id,
		name: display_name,
		source: "local".to_string(),
		project_id: None,
		version_id: None,
		version_number: None,
		filename: file_name,
		addon_type: inferred_type,
		game_version: None,
		file_size: size,
		installed_at: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
		enabled: true,
		dependencies: Vec::new(),
		icon_url: None,
	};

	registry.addons.push(new_entry.clone());
	save_registry(&server_dir, &registry).await.ok();

	Ok(new_entry)
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_delete_addon(
	server_id: Option<String>,
	addon_id: String,
	filename: String,
) -> Result<()> {
	validate_safe_filename(&filename)?;
	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let (engine, _) = read_server_engine(&server_dir).await;

	let folders = [
		resolve_addon_dir(&server_dir, &engine, "plugin"),
		resolve_addon_dir(&server_dir, &engine, "mod"),
		resolve_addon_dir(&server_dir, &engine, "datapack"),
	];

	let disabled_filename = format!("{}.disabled", filename);

	for folder in &folders {
		let path = folder.join(&filename);
		if path.exists() {
			fs::remove_file(&path).await.ok();
		}
		let disabled_path = folder.join(&disabled_filename);
		if disabled_path.exists() {
			fs::remove_file(&disabled_path).await.ok();
		}
	}

	let mut registry = load_registry(&server_dir).await;
	registry.addons.retain(|a| a.id != addon_id && a.filename != filename);
	save_registry(&server_dir, &registry).await.ok();

	Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_toggle_addon(
	server_id: Option<String>,
	addon_id: String,
	filename: String,
	enabled: bool,
) -> Result<()> {
	validate_safe_filename(&filename)?;
	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let (engine, _) = read_server_engine(&server_dir).await;

	let folders = [
		resolve_addon_dir(&server_dir, &engine, "plugin"),
		resolve_addon_dir(&server_dir, &engine, "mod"),
		resolve_addon_dir(&server_dir, &engine, "datapack"),
	];

	for folder in &folders {
		let current_path = folder.join(&filename);
		if current_path.exists() {
			if enabled && filename.ends_with(".disabled") {
				let target_name = filename.trim_end_matches(".disabled").to_string();
				let target_path = folder.join(&target_name);
				fs::rename(&current_path, &target_path).await.ok();
			} else if !enabled && !filename.ends_with(".disabled") {
				let target_name = format!("{}.disabled", filename);
				let target_path = folder.join(&target_name);
				fs::rename(&current_path, &target_path).await.ok();
			}
		}
	}

	let mut registry = load_registry(&server_dir).await;
	if let Some(entry) = registry.addons.iter_mut().find(|a| a.id == addon_id || a.filename == filename) {
		entry.enabled = enabled;
		if enabled {
			entry.filename = entry.filename.trim_end_matches(".disabled").to_string();
		} else if !entry.filename.ends_with(".disabled") {
			entry.filename = format!("{}.disabled", entry.filename);
		}
	}
	save_registry(&server_dir, &registry).await.ok();

	Ok(())
}

#[cfg_attr(feature = "tauri", tauri::command)]
pub async fn host_update_addon_entry(
	server_id: Option<String>,
	addon_id: String,
	icon_url: Option<String>,
	project_id: Option<String>,
	name: Option<String>,
) -> Result<()> {
	let state = State::get().await?;
	let server_dir = get_target_server_dir(&state, server_id.as_deref()).await?;
	let mut registry = load_registry(&server_dir).await;
	if let Some(entry) = registry.addons.iter_mut().find(|a| a.id == addon_id || a.filename == addon_id) {
		if let Some(url) = icon_url {
			entry.icon_url = Some(url);
		}
		if let Some(pid) = project_id {
			entry.project_id = Some(pid);
		}
		if let Some(n) = name {
			entry.name = n;
		}
	}
	save_registry(&server_dir, &registry).await.ok();
	Ok(())
}
