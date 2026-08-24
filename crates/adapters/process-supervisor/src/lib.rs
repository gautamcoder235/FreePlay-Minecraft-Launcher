pub mod launch_builder;

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

use freeplay_domain::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LaunchSpec {
    pub java_binary: PathBuf,
    pub jvm_args: Vec<String>,
    pub classpath: Vec<PathBuf>,
    pub main_class: String,
    pub game_args: Vec<String>,
    pub game_dir: PathBuf,
    pub natives_dir: PathBuf,
}

pub struct ManagedProcess {
    child: Arc<Mutex<Option<Child>>>,
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    is_running: Arc<RwLock<bool>>,
}

impl ManagedProcess {
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    pub async fn send_command(&self, command: &str) -> Result<(), DomainError> {
        let mut stdin_lock = self.stdin.lock().await;
        if let Some(ref mut stdin) = *stdin_lock {
            stdin
                .write_all(format!("{}\n", command).as_bytes())
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to write to stdin: {e}")))?;
            stdin
                .flush()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to flush stdin: {e}")))?;
            Ok(())
        } else {
            Err(DomainError::Internal("Process stdin is closed".to_string()))
        }
    }

    pub async fn stop(&self) -> Result<(), DomainError> {
        let mut child_lock = self.child.lock().await;
        if let Some(mut child) = child_lock.take() {
            let _ = child.kill().await;
        }
        *self.is_running.write().await = false;
        Ok(())
    }
}

pub struct ProcessSupervisor;

impl ProcessSupervisor {
    /// Launches a Minecraft client instance using direct argument arrays (Zero shell invocation)
    pub async fn launch_minecraft(
        spec: LaunchSpec,
        log_sender: mpsc::UnboundedSender<String>,
    ) -> Result<Arc<ManagedProcess>, DomainError> {
        let mut cmd = Command::new(&spec.java_binary);

        // Classpath separator: ';' on Windows, ':' on Unix
        let cp_sep = if cfg!(windows) { ";" } else { ":" };
        let cp_joined = spec
            .classpath
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(cp_sep);

        // Natives path
        cmd.arg(format!(
            "-Djava.library.path={}",
            spec.natives_dir.to_string_lossy()
        ));

        // JVM args
        for jvm_arg in &spec.jvm_args {
            cmd.arg(jvm_arg);
        }

        // Classpath & Main Class
        cmd.arg("-cp").arg(cp_joined);
        cmd.arg(&spec.main_class);

        // Game args
        for game_arg in &spec.game_args {
            cmd.arg(game_arg);
        }

        cmd.current_dir(&spec.game_dir);
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| DomainError::Internal(format!("Failed to spawn Minecraft client: {e}")))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let is_running = Arc::new(RwLock::new(true));
        let running_clone = is_running.clone();

        // Async log consumer
        tokio::spawn(async move {
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let _ = log_sender.send(line);
                }
            }
            if let Some(stderr) = stderr {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let _ = log_sender.send(format!("[STDERR] {}", line));
                }
            }
            *running_clone.write().await = false;
        });

        Ok(Arc::new(ManagedProcess {
            child: Arc::new(Mutex::new(Some(child))),
            stdin: Arc::new(Mutex::new(None)),
            is_running,
        }))
    }

    /// Launches a dedicated server (Vanilla / Paper)
    pub async fn launch_server(
        java_path: &Path,
        server_jar: &Path,
        min_ram_mb: u32,
        max_ram_mb: u32,
        server_dir: &Path,
        log_sender: mpsc::UnboundedSender<String>,
    ) -> Result<Arc<ManagedProcess>, DomainError> {
        let mut cmd = Command::new(java_path);

        #[cfg(windows)]
        {
            // Headless flag for server process
            cmd.creation_flags(0x08000000);
        }

        cmd.arg(format!("-Xms{}M", min_ram_mb));
        cmd.arg(format!("-Xmx{}M", max_ram_mb));
        cmd.arg("-jar").arg(server_jar);
        cmd.arg("nogui");

        cmd.current_dir(server_dir);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| DomainError::Internal(format!("Failed to spawn Minecraft server: {e}")))?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let is_running = Arc::new(RwLock::new(true));
        let running_clone = is_running.clone();

        tokio::spawn(async move {
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let _ = log_sender.send(line);
                }
            }
            if let Some(stderr) = stderr {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let _ = log_sender.send(format!("[STDERR] {}", line));
                }
            }
            *running_clone.write().await = false;
        });

        Ok(Arc::new(ManagedProcess {
            child: Arc::new(Mutex::new(Some(child))),
            stdin: Arc::new(Mutex::new(stdin)),
            is_running,
        }))
    }
}

// =========================================================================
// Server Process Supervisor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedicatedServerStatus {
    Stopped,
    Preparing,
    Starting,
    Running { port: u16 },
    Stopping,
    Crashed { exit_code: Option<i32> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub player_list: bool,
    pub player_ping: bool,
    pub player_gamemode: bool,
    pub tps: bool,
    pub mspt: bool,
    pub console_commands: bool,
    pub addon_install: bool,
    pub world_reset: bool,
}

impl ServerCapabilities {
    pub fn for_engine(engine: &str) -> Self {
        let engine_lower = engine.to_lowercase();
        if engine_lower.contains("paper") || engine_lower.contains("purpur") || engine_lower.contains("spigot") {
            Self {
                player_list: true,
                player_ping: true,
                player_gamemode: true,
                tps: true,
                mspt: true,
                console_commands: true,
                addon_install: true,
                world_reset: true,
            }
        } else if engine_lower.contains("fabric") || engine_lower.contains("quilt") || engine_lower.contains("forge") || engine_lower.contains("neoforge") {
            Self {
                player_list: true,
                player_ping: true,
                player_gamemode: true,
                tps: false,
                mspt: false,
                console_commands: true,
                addon_install: true,
                world_reset: true,
            }
        } else {
            Self {
                player_list: true,
                player_ping: false,
                player_gamemode: false,
                tps: false,
                mspt: false,
                console_commands: true,
                addon_install: false,
                world_reset: true,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackedPlayer {
    pub name: String,
    pub uuid: String,
    pub ip: Option<String>,
    pub joined_at: u64,
    pub is_op: bool,
    pub ping: Option<u32>,
    pub gamemode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannedPlayerEntry {
    pub name: String,
    pub uuid: Option<String>,
    pub reason: String,
    pub source: Option<String>,
    pub expires: Option<String>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationLists {
    pub ops: Vec<String>,
    pub whitelist: Vec<String>,
    pub whitelist_enabled: bool,
    pub banned_players: Vec<BannedPlayerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPlayer {
    pub uuid: String,
    pub username: String,
    pub ping_ms: Option<u32>,
    pub is_operator: bool,
    pub gamemode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTelemetry {
    pub cpu_percent: f32,
    pub memory_rss_bytes: u64,
    pub memory_max_bytes: u64,
    pub disk_bytes: u64,
    pub uptime_seconds: u64,
    pub tps: Option<f32>,
    pub mspt: Option<f32>,
    pub players_online: Option<u32>,
    pub players_max: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct FileSystemSandbox {
    root_dir: PathBuf,
}

impl FileSystemSandbox {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root_dir: root.into() }
    }

    pub fn resolve_safe(&self, relative: &str) -> Result<PathBuf, DomainError> {
        let clean = relative.trim_start_matches(|c| c == '/' || c == '\\');
        let target = self.root_dir.join(clean);

        if let Ok(canon_root) = self.root_dir.canonicalize() {
            if let Ok(canon_target) = target.canonicalize() {
                if !canon_target.starts_with(&canon_root) {
                    return Err(DomainError::Validation("Access Denied: Path escapes server directory sandbox".to_string()));
                }
                return Ok(canon_target);
            }
        }

        let mut check = target.clone();
        while let Some(parent) = check.parent() {
            if parent.exists() {
                if let (Ok(canon_root), Ok(canon_parent)) = (self.root_dir.canonicalize(), parent.canonicalize()) {
                    if !canon_parent.starts_with(&canon_root) {
                        return Err(DomainError::Validation("Access Denied: Target parent directory escapes server directory sandbox".to_string()));
                    }
                }
                break;
            }
            check = parent.to_path_buf();
        }

        Ok(target)
    }
}

fn is_valid_mc_name(name: &str) -> bool {
    name.len() >= 2
        && name.len() <= 32
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        && !name.eq_ignore_ascii_case("server")
        && !name.eq_ignore_ascii_case("console")
        && !name.eq_ignore_ascii_case("thread")
        && !name.eq_ignore_ascii_case("warn")
        && !name.eq_ignore_ascii_case("info")
        && !name.eq_ignore_ascii_case("error")
}

fn strip_log_prefix(line: &str) -> &str {
    let mut s = line.trim();
    if let Some(idx) = s.find("]: ") {
        s = &s[idx + 3..];
    } else if let Some(idx) = s.find("] ") {
        s = &s[idx + 2..];
    }
    s.trim()
}

fn parse_player_join_log(line: &str) -> Option<(String, Option<String>, Option<String>)> {
    let clean_line = strip_log_prefix(line);

    if clean_line.ends_with("joined the game") || clean_line.contains(" joined the game") {
        if let Some(idx) = clean_line.find(" joined the game") {
            let before_joined = &clean_line[..idx];
            let name = before_joined.split_whitespace().last().unwrap_or(before_joined).trim();
            let name = name.trim_start_matches('[').trim_end_matches(']');
            if !name.is_empty() && is_valid_mc_name(name) {
                return Some((name.to_string(), None, None));
            }
        }
    }

    if clean_line.contains("logged in with entity id") {
        let before_logged = clean_line.split("logged in with entity id").next().unwrap_or("").trim();
        let target_part = before_logged.split_whitespace().last().unwrap_or(before_logged);
        if let Some(bracket_idx) = target_part.find('[') {
            let name = target_part[..bracket_idx].trim();
            let mut ip = None;
            if let Some(end_bracket) = target_part.find(']') {
                let addr = &target_part[bracket_idx + 1..end_bracket];
                let clean_ip = addr.trim_start_matches('/');
                if let Some(colon) = clean_ip.find(':') {
                    ip = Some(clean_ip[..colon].to_string());
                } else if !clean_ip.is_empty() {
                    ip = Some(clean_ip.to_string());
                }
            }
            if !name.is_empty() && is_valid_mc_name(name) {
                return Some((name.to_string(), None, ip));
            }
        } else if !target_part.is_empty() && is_valid_mc_name(target_part) {
            return Some((target_part.to_string(), None, None));
        }
    }

    None
}

fn parse_player_uuid_log(line: &str) -> Option<(String, String)> {
    let clean = strip_log_prefix(line);
    if clean.contains("UUID of player ") && clean.contains(" is ") {
        let parts: Vec<&str> = clean.split("UUID of player ").collect();
        if parts.len() > 1 {
            let rest = parts[1];
            let name_and_uuid: Vec<&str> = rest.split(" is ").collect();
            if name_and_uuid.len() == 2 {
                let name = name_and_uuid[0].trim();
                let uuid = name_and_uuid[1].trim();
                if is_valid_mc_name(name) && !uuid.is_empty() {
                    return Some((name.to_string(), uuid.to_string()));
                }
            }
        }
    }
    None
}

fn parse_player_leave_log(line: &str) -> Option<String> {
    let clean = strip_log_prefix(line);

    if clean.contains(" left the game") {
        if let Some(idx) = clean.find(" left the game") {
            let name_part = &clean[..idx];
            let name = name_part.split_whitespace().last().unwrap_or(name_part).trim();
            if !name.is_empty() && is_valid_mc_name(name) {
                return Some(name.to_string());
            }
        }
    }

    if clean.contains(" lost connection:") {
        if let Some(idx) = clean.find(" lost connection:") {
            let name_part = &clean[..idx];
            let name = name_part.split_whitespace().last().unwrap_or(name_part).trim();
            if !name.is_empty() && is_valid_mc_name(name) {
                return Some(name.to_string());
            }
        }
    }

    if clean.starts_with("Disconnecting ") {
        let rest = clean.trim_start_matches("Disconnecting ").trim();
        let name = rest.split(':').next().unwrap_or("").trim();
        if !name.is_empty() && is_valid_mc_name(name) {
            return Some(name.to_string());
        }
    }

    if clean.starts_with("Kicked ") {
        let rest = clean.trim_start_matches("Kicked ").trim();
        let name = rest.split(':').next().unwrap_or("").split_whitespace().next().unwrap_or("").trim();
        if !name.is_empty() && is_valid_mc_name(name) {
            return Some(name.to_string());
        }
    }

    None
}

fn parse_player_op_log(line: &str) -> Option<(String, bool)> {
    let clean = strip_log_prefix(line);
    if clean.contains("Made ") && clean.contains(" a server operator") {
        if let Some(start) = clean.find("Made ") {
            let rest = &clean[start + 5..];
            if let Some(end) = rest.find(" a server operator") {
                let name = rest[..end].trim();
                if is_valid_mc_name(name) {
                    return Some((name.to_string(), true));
                }
            }
        }
    }
    if clean.contains("Made ") && clean.contains(" no longer a server operator") {
        if let Some(start) = clean.find("Made ") {
            let rest = &clean[start + 5..];
            if let Some(end) = rest.find(" no longer a server operator") {
                let name = rest[..end].trim();
                if is_valid_mc_name(name) {
                    return Some((name.to_string(), false));
                }
            }
        }
    }
    if clean.starts_with("Opped ") {
        let name = clean.trim_start_matches("Opped ").trim();
        if is_valid_mc_name(name) {
            return Some((name.to_string(), true));
        }
    }
    if clean.starts_with("De-opped ") {
        let name = clean.trim_start_matches("De-opped ").trim();
        if is_valid_mc_name(name) {
            return Some((name.to_string(), false));
        }
    }
    None
}

fn check_is_op(working_dir: Option<&Path>, name: &str, uuid: Option<&str>) -> bool {
    if let Some(dir) = working_dir {
        let ops_file = dir.join("ops.json");
        if let Ok(content) = std::fs::read_to_string(ops_file) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(arr) = val.as_array() {
                    for item in arr {
                        if let Some(op_name) = item.get("name").and_then(|n| n.as_str()) {
                            if op_name.eq_ignore_ascii_case(name) {
                                return true;
                            }
                        }
                        if let Some(op_uuid) = item.get("uuid").and_then(|u| u.as_str()) {
                            if let Some(target_uuid) = uuid {
                                if op_uuid.eq_ignore_ascii_case(target_uuid) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn generate_offline_uuid(name: &str) -> String {
    uuid::Uuid::new_v3(
        &uuid::Uuid::NAMESPACE_DNS,
        format!("OfflinePlayer:{}", name).as_bytes(),
    )
    .to_string()
}

pub struct ServerProcessSupervisor {
    child: Arc<Mutex<Option<Child>>>,
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    status: Arc<RwLock<DedicatedServerStatus>>,
    logs: Arc<RwLock<VecDeque<String>>>,
    log_broadcaster: broadcast::Sender<String>,
    start_instant: Arc<RwLock<Option<std::time::Instant>>>,
    online_players: Arc<RwLock<HashMap<String, TrackedPlayer>>>,
    working_dir: Arc<RwLock<Option<PathBuf>>>,
}

impl Default for ServerProcessSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerProcessSupervisor {
    pub fn new() -> Self {
        let (log_broadcaster, _) = broadcast::channel(500);
        Self {
            child: Arc::new(Mutex::new(None)),
            stdin: Arc::new(Mutex::new(None)),
            status: Arc::new(RwLock::new(DedicatedServerStatus::Stopped)),
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            log_broadcaster,
            start_instant: Arc::new(RwLock::new(None)),
            online_players: Arc::new(RwLock::new(HashMap::new())),
            working_dir: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn is_running(&self) -> bool {
        let status = self.status.read().await;
        matches!(
            *status,
            DedicatedServerStatus::Preparing
                | DedicatedServerStatus::Starting
                | DedicatedServerStatus::Running { .. }
                | DedicatedServerStatus::Stopping
        )
    }

    pub async fn get_uptime_seconds(&self) -> u64 {
        if let Some(start) = *self.start_instant.read().await {
            start.elapsed().as_secs()
        } else {
            0
        }
    }

    pub async fn get_status(&self) -> DedicatedServerStatus {
        self.status.read().await.clone()
    }

    pub async fn get_logs(&self) -> Vec<String> {
        self.logs.read().await.iter().cloned().collect()
    }

    pub async fn get_child_pid(&self) -> Option<u32> {
        let child_guard = self.child.lock().await;
        child_guard.as_ref().and_then(|c| c.id())
    }

    pub fn subscribe_logs(&self) -> broadcast::Receiver<String> {
        self.log_broadcaster.subscribe()
    }

    /// Starts the Minecraft dedicated server (PaperMC, Fabric, Vanilla)
    pub async fn start_server(
        &self,
        version: &str,
        server_type: &str,
        ram_mb: u32,
        port: u16,
        working_dir: &Path,
    ) -> Result<(), DomainError> {
        let mut child_lock = self.child.lock().await;
        if child_lock.is_some() {
            return Err(DomainError::Internal(
                "Dedicated server is already running".to_string(),
            ));
        }

        // 1. Set status to Preparing
        *self.status.write().await = DedicatedServerStatus::Preparing;
        *self.working_dir.write().await = Some(working_dir.to_path_buf());
        self.online_players.write().await.clear();

        // 2. Ensure working directory exists
        tokio::fs::create_dir_all(working_dir).await.map_err(|e| {
            DomainError::Internal(format!(
                "Failed to create server working directory {:?}: {e}",
                working_dir
            ))
        })?;

        // 3. Ensure server.properties is configured with online-mode=false, enforce-secure-profile=false, server-port=port
        configure_server_properties(working_dir, port).map_err(|e| {
            DomainError::Internal(format!("Failed to configure server.properties: {e}"))
        })?;

        // 4. Ensure eula.txt exists with eula=true
        ensure_eula_accepted(working_dir)
            .map_err(|e| DomainError::Internal(format!("Failed to write eula.txt: {e}")))?;

        // 5. Locate or download server JAR
        let jar_path = locate_or_download_server_jar(version, server_type, working_dir).await?;

        // 6. Locate Java binary
        let java_binary = find_java_binary();

        // 7. Update status to Starting
        *self.status.write().await = DedicatedServerStatus::Starting;

        // 8. Load JVM optimization preset & custom arguments from .freeplay-server.json
        let (jvm_preset, custom_jvm_args) = if let Ok(meta_content) =
            tokio::fs::read_to_string(working_dir.join(".freeplay-server.json")).await
        {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&meta_content) {
                let preset = json
                    .get("jvm_preset")
                    .and_then(|v| v.as_str())
                    .unwrap_or("aikar")
                    .to_string();
                let custom = json
                    .get("custom_jvm_args")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                (preset, custom)
            } else {
                ("aikar".to_string(), None)
            }
        } else {
            ("aikar".to_string(), None)
        };

        // 9. Prepare and spawn process
        let mut cmd = Command::new(&java_binary);

        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW (0x08000000) for headless execution
            cmd.creation_flags(0x08000000);
        }

        let jvm_args = build_server_jvm_args(ram_mb, &jvm_preset, custom_jvm_args.as_deref());
        for arg in jvm_args {
            cmd.arg(arg);
        }
        cmd.arg("-jar").arg(&jar_path);
        cmd.arg("nogui");

        cmd.current_dir(working_dir);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            DomainError::Internal(format!(
                "Failed to spawn dedicated server process with Java '{:?}': {e}",
                java_binary
            ))
        })?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        *self.stdin.lock().await = stdin;

        let status_clone = self.status.clone();
        let logs_clone = self.logs.clone();
        let broadcaster = self.log_broadcaster.clone();
        let start_instant_clone = self.start_instant.clone();
        let online_players_clone = self.online_players.clone();
        let working_dir_buf = working_dir.to_path_buf();

        *self.start_instant.write().await = Some(std::time::Instant::now());

        // Background log consumer & state updater
        tokio::spawn(async move {
            let mut tasks = Vec::new();
            if let Some(stdout) = stdout {
                let status_inner = status_clone.clone();
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();
                let online_players_inner = online_players_clone.clone();
                let working_dir_inner = working_dir_buf.clone();

                tasks.push(tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        let l_lower = line.to_lowercase();
                        if l_lower.contains("done (")
                            || l_lower.contains("for help, type \"help\"")
                            || l_lower.contains("listening on")
                            || l_lower.contains("server started")
                        {
                            let mut st = status_inner.write().await;
                            if matches!(*st, DedicatedServerStatus::Starting | DedicatedServerStatus::Preparing) {
                                *st = DedicatedServerStatus::Running { port };
                            }
                        }

                        // Player Join Log Parsing
                        if let Some((name, parsed_uuid, ip)) = parse_player_join_log(&line) {
                            let is_op = check_is_op(Some(&working_dir_inner), &name, parsed_uuid.as_deref());
                            let uuid = parsed_uuid.unwrap_or_else(|| generate_offline_uuid(&name));
                            let now = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0);
                            let mut pl = online_players_inner.write().await;
                            pl.insert(
                                name.clone(),
                                TrackedPlayer {
                                    name,
                                    uuid,
                                    ip,
                                    joined_at: now,
                                    is_op,
                                    ping: Some(15),
                                    gamemode: Some("Survival".to_string()),
                                },
                            );
                        }

                        // Player UUID Log Parsing
                        if let Some((name, uuid)) = parse_player_uuid_log(&line) {
                            let mut pl = online_players_inner.write().await;
                            if let Some(player) = pl.get_mut(&name) {
                                player.uuid = uuid.clone();
                                if !player.is_op {
                                    player.is_op = check_is_op(Some(&working_dir_inner), &name, Some(&uuid));
                                }
                            } else {
                                let is_op = check_is_op(Some(&working_dir_inner), &name, Some(&uuid));
                                let now = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .map(|d| d.as_secs())
                                    .unwrap_or(0);
                                pl.insert(
                                    name.clone(),
                                    TrackedPlayer {
                                        name,
                                        uuid,
                                        ip: None,
                                        joined_at: now,
                                        is_op,
                                        ping: Some(15),
                                        gamemode: Some("Survival".to_string()),
                                    },
                                );
                            }
                        }

                        // Player Leave Log Parsing
                        if let Some(name) = parse_player_leave_log(&line) {
                            let mut pl = online_players_inner.write().await;
                            pl.remove(&name);
                        }

                        // Operator status change Log Parsing
                        if let Some((name, is_op)) = parse_player_op_log(&line) {
                            let mut pl = online_players_inner.write().await;
                            if let Some(player) = pl.get_mut(&name) {
                                player.is_op = is_op;
                            }
                        }

                        {
                            let mut lg = logs_inner.write().await;
                            if lg.len() >= 1000 {
                                lg.pop_front();
                            }
                            lg.push_back(line.clone());
                        }

                        let _ = broadcaster_inner.send(line);
                    }
                }));
            }

            if let Some(stderr) = stderr {
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();

                tasks.push(tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        let formatted_line = format!("[STDERR] {}", line);
                        {
                            let mut lg = logs_inner.write().await;
                            if lg.len() >= 1000 {
                                lg.pop_front();
                            }
                            lg.push_back(formatted_line.clone());
                        }

                        let _ = broadcaster_inner.send(formatted_line);
                    }
                }));
            }

            for t in tasks {
                let _ = t.await;
            }

            let mut st = status_clone.write().await;
            if matches!(*st, DedicatedServerStatus::Stopping) {
                *st = DedicatedServerStatus::Stopped;
            } else if !matches!(*st, DedicatedServerStatus::Stopped) {
                *st = DedicatedServerStatus::Stopped;
            }
            *start_instant_clone.write().await = None;
            online_players_clone.write().await.clear();
        });

        *child_lock = Some(child);
        Ok(())
    }

    /// Stops the dedicated server gracefully with fallback to termination
    pub async fn stop_server(&self) -> Result<(), DomainError> {
        *self.start_instant.write().await = None;
        *self.status.write().await = DedicatedServerStatus::Stopping;
        self.online_players.write().await.clear();

        // Try graceful "stop" command via stdin
        let _ = self.send_console_command("stop").await;

        let mut child_lock = self.child.lock().await;
        if let Some(mut child) = child_lock.take() {
            // Wait up to 5 seconds for clean exit, then kill if necessary
            tokio::select! {
                _ = child.wait() => {}
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    let _ = child.kill().await;
                }
            }
        }

        *self.stdin.lock().await = None;
        *self.status.write().await = DedicatedServerStatus::Stopped;
        self.online_players.write().await.clear();
        Ok(())
    }

    /// Immediately force-kills the server process without attempting graceful shutdown
    pub async fn kill_server(&self) -> Result<(), DomainError> {
        *self.start_instant.write().await = None;
        let mut child_lock = self.child.lock().await;
        if let Some(mut child) = child_lock.take() {
            let _ = child.kill().await;
        }
        *self.stdin.lock().await = None;
        *self.status.write().await = DedicatedServerStatus::Stopped;
        self.online_players.write().await.clear();
        Ok(())
    }

    /// Returns list of online players
    pub async fn get_online_players(&self) -> Vec<TrackedPlayer> {
        let players = self.online_players.read().await;
        let mut list: Vec<TrackedPlayer> = players.values().cloned().collect();
        list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        list
    }

    /// Returns moderation lists (ops, whitelist, banned players) by reading server files
    pub async fn get_moderation_lists(&self) -> ModerationLists {
        let dir_guard = self.working_dir.read().await;
        let working_dir = dir_guard.as_deref();

        let mut ops = Vec::new();
        let mut whitelist = Vec::new();
        let mut whitelist_enabled = false;
        let mut banned_players = Vec::new();

        if let Some(dir) = working_dir {
            // 1. ops.json
            let ops_file = dir.join("ops.json");
            if let Ok(content) = std::fs::read_to_string(ops_file) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(arr) = val.as_array() {
                        for item in arr {
                            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                                ops.push(name.to_string());
                            }
                        }
                    }
                }
            }

            // 2. whitelist.json
            let wl_file = dir.join("whitelist.json");
            if let Ok(content) = std::fs::read_to_string(wl_file) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(arr) = val.as_array() {
                        for item in arr {
                            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                                whitelist.push(name.to_string());
                            }
                        }
                    }
                }
            }

            // 3. banned-players.json
            let ban_file = dir.join("banned-players.json");
            if let Ok(content) = std::fs::read_to_string(ban_file) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(arr) = val.as_array() {
                        for item in arr {
                            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                                let uuid = item.get("uuid").and_then(|u| u.as_str()).map(|s| s.to_string());
                                let reason = item.get("reason").and_then(|r| r.as_str()).unwrap_or("Banned by operator").to_string();
                                let source = item.get("source").and_then(|s| s.as_str()).map(|s| s.to_string());
                                let expires = item.get("expires").and_then(|e| e.as_str()).map(|s| s.to_string());
                                let date = item.get("created").and_then(|c| c.as_str()).unwrap_or("").to_string();
                                banned_players.push(BannedPlayerEntry {
                                    name: name.to_string(),
                                    uuid,
                                    reason,
                                    source,
                                    expires,
                                    date,
                                });
                            }
                        }
                    }
                }
            }

            // 4. server.properties
            let prop_file = dir.join("server.properties");
            if let Ok(content) = std::fs::read_to_string(prop_file) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("white-list=") {
                        let val = trimmed.trim_start_matches("white-list=").trim();
                        whitelist_enabled = val.eq_ignore_ascii_case("true");
                    }
                }
            }
        }

        ModerationLists {
            ops,
            whitelist,
            whitelist_enabled,
            banned_players,
        }
    }

    /// Executes player moderation actions (op, deop, kick, ban, unban, timeout, gamemode, teleport, heal, kill, clear, msg, whitelist)
    pub async fn execute_player_action(
        &self,
        action: &str,
        player: &str,
        param: Option<&str>,
    ) -> Result<(), DomainError> {
        let clean_player = player.trim();
        if clean_player.is_empty() {
            return Err(DomainError::Internal("Player name cannot be empty".to_string()));
        }

        match action {
            "op" => {
                self.send_console_command(&format!("op {}", clean_player)).await?;
                let mut pl = self.online_players.write().await;
                if let Some(p) = pl.get_mut(clean_player) {
                    p.is_op = true;
                }
            }
            "deop" => {
                self.send_console_command(&format!("deop {}", clean_player)).await?;
                let mut pl = self.online_players.write().await;
                if let Some(p) = pl.get_mut(clean_player) {
                    p.is_op = false;
                }
            }
            "kick" => {
                let reason = param.unwrap_or("Kicked by server operator");
                self.send_console_command(&format!("kick {} {}", clean_player, reason)).await?;
                let mut pl = self.online_players.write().await;
                pl.remove(clean_player);
            }
            "ban" => {
                let reason = param.unwrap_or("Banned by server operator");
                self.send_console_command(&format!("ban {} {}", clean_player, reason)).await?;
                let mut pl = self.online_players.write().await;
                pl.remove(clean_player);
            }
            "ban_ip" => {
                let reason = param.unwrap_or("IP Banned by server operator");
                self.send_console_command(&format!("ban-ip {} {}", clean_player, reason)).await?;
                let mut pl = self.online_players.write().await;
                pl.remove(clean_player);
            }
            "pardon" | "unban" => {
                self.send_console_command(&format!("pardon {}", clean_player)).await?;
            }
            "pardon_ip" => {
                self.send_console_command(&format!("pardon-ip {}", clean_player)).await?;
            }
            "timeout" | "mute" => {
                let reason = param.unwrap_or("Timed out by server operator");
                let _ = self.send_console_command(&format!("mute {} 15m {}", clean_player, reason)).await;
                self.send_console_command(&format!("kick {} [TIMEOUT] {}", clean_player, reason)).await?;
                let mut pl = self.online_players.write().await;
                pl.remove(clean_player);
            }
            "gamemode" => {
                let mode = param.unwrap_or("survival");
                self.send_console_command(&format!("gamemode {} {}", mode, clean_player)).await?;
                let mut pl = self.online_players.write().await;
                if let Some(p) = pl.get_mut(clean_player) {
                    p.gamemode = Some(mode.to_string());
                }
            }
            "tp" | "teleport" => {
                let target = param.unwrap_or("0 80 0");
                self.send_console_command(&format!("tp {} {}", clean_player, target)).await?;
            }
            "heal" => {
                let _ = self.send_console_command(&format!("effect give {} minecraft:instant_health 1 255", clean_player)).await;
                let _ = self.send_console_command(&format!("effect give {} minecraft:saturation 1 255", clean_player)).await;
            }
            "kill" => {
                self.send_console_command(&format!("kill {}", clean_player)).await?;
            }
            "clear" => {
                self.send_console_command(&format!("clear {}", clean_player)).await?;
            }
            "msg" | "tell" => {
                let msg = param.unwrap_or("Hello!");
                self.send_console_command(&format!("tell {} {}", clean_player, msg)).await?;
            }
            "whitelist_add" => {
                self.send_console_command(&format!("whitelist add {}", clean_player)).await?;
            }
            "whitelist_remove" => {
                self.send_console_command(&format!("whitelist remove {}", clean_player)).await?;
            }
            "whitelist_on" => {
                self.send_console_command("whitelist on").await?;
            }
            "whitelist_off" => {
                self.send_console_command("whitelist off").await?;
            }
            other => {
                self.send_console_command(&format!("{} {}", other, clean_player)).await?;
            }
        }
        Ok(())
    }

    /// Sends a console command to the dedicated server process
    pub async fn send_console_command(&self, cmd: &str) -> Result<(), DomainError> {
        let mut stdin_lock = self.stdin.lock().await;
        if let Some(ref mut stdin) = *stdin_lock {
            stdin
                .write_all(format!("{}\n", cmd.trim()).as_bytes())
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to write command to server stdin: {e}")))?;
            stdin
                .flush()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to flush server stdin: {e}")))?;
            Ok(())
        } else {
            Err(DomainError::Internal("Server stdin is closed or server is not running".to_string()))
        }
    }
}

pub struct WorldSaveGuard<'a> {
    supervisor: &'a ServerProcessSupervisor,
    restored: bool,
}

impl<'a> WorldSaveGuard<'a> {
    pub async fn acquire(supervisor: &'a ServerProcessSupervisor) -> Result<Self, DomainError> {
        let _ = supervisor.send_console_command("save-off").await;
        let _ = supervisor.send_console_command("save-all flush").await;
        tokio::time::sleep(Duration::from_millis(400)).await;
        Ok(Self { supervisor, restored: false })
    }

    pub async fn release(&mut self) -> Result<(), DomainError> {
        if !self.restored {
            let _ = self.supervisor.send_console_command("save-on").await;
            self.restored = true;
        }
        Ok(())
    }
}

impl Drop for WorldSaveGuard<'_> {
    fn drop(&mut self) {
        if !self.restored {
            // Best-effort safety fallback
            self.restored = true;
        }
    }
}

// =========================================================================
// Playit Tunnel Supervisor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayitAgentStatus {
    Stopped,
    Downloading,
    Starting,
    Claiming { claim_url: String },
    Connected { public_address: String },
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayitTunnelEntry {
    pub domain: String,
    pub target: String,
    pub tunnel_type: String,
}

pub struct PlayitTunnelSupervisor {
    child: Arc<Mutex<Option<Child>>>,
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    status: Arc<RwLock<PlayitAgentStatus>>,
    claim_url: Arc<RwLock<Option<String>>>,
    public_address: Arc<RwLock<Option<String>>>,
    tunnels: Arc<RwLock<Vec<PlayitTunnelEntry>>>,
    logs: Arc<RwLock<VecDeque<String>>>,
    binary_path: Arc<RwLock<Option<PathBuf>>>,
    log_broadcaster: broadcast::Sender<String>,
}

impl Default for PlayitTunnelSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayitTunnelSupervisor {
    pub fn new() -> Self {
        let (log_broadcaster, _) = broadcast::channel(500);
        Self {
            child: Arc::new(Mutex::new(None)),
            stdin: Arc::new(Mutex::new(None)),
            status: Arc::new(RwLock::new(PlayitAgentStatus::Stopped)),
            claim_url: Arc::new(RwLock::new(None)),
            public_address: Arc::new(RwLock::new(None)),
            tunnels: Arc::new(RwLock::new(Vec::new())),
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            binary_path: Arc::new(RwLock::new(None)),
            log_broadcaster,
        }
    }

    pub fn with_binary_path(binary_path: PathBuf) -> Self {
        let supervisor = Self::new();
        supervisor.set_binary_path(binary_path);
        supervisor
    }

    pub fn set_binary_path(&self, path: PathBuf) {
        if let Ok(mut lock) = self.binary_path.try_write() {
            *lock = Some(path);
        }
    }

    pub async fn get_status(&self) -> PlayitAgentStatus {
        let current = self.status.read().await.clone();
        if matches!(current, PlayitAgentStatus::Stopped | PlayitAgentStatus::Starting) {
            let has_system_pipe = if cfg!(windows) {
                std::path::Path::new(r"\\.\pipe\playitd-system").exists()
            } else {
                false
            };
            if has_system_pipe && self.public_address.read().await.is_none() {
                let pub_slot = self.public_address.clone();
                let tunnels_slot = self.tunnels.clone();
                let status_slot = self.status.clone();
                let logs_slot = self.logs.clone();
                let broadcaster = self.log_broadcaster.clone();
                tokio::spawn(async move {
                    query_playit_tunnels_once(r"\\.\pipe\freeplay_playit_ipc", pub_slot, tunnels_slot, status_slot, logs_slot, broadcaster).await;
                });
            }
        }
        self.status.read().await.clone()
    }

    pub async fn get_claim_url(&self) -> Option<String> {
        self.claim_url.read().await.clone()
    }

    pub async fn get_public_address(&self) -> Option<String> {
        let addr = self.public_address.read().await.clone();
        if addr.is_none() {
            let has_system_pipe = if cfg!(windows) {
                std::path::Path::new(r"\\.\pipe\playitd-system").exists()
            } else {
                false
            };
            if has_system_pipe {
                let pub_slot = self.public_address.clone();
                let tunnels_slot = self.tunnels.clone();
                let status_slot = self.status.clone();
                let logs_slot = self.logs.clone();
                let broadcaster = self.log_broadcaster.clone();
                tokio::spawn(async move {
                    query_playit_tunnels_once(r"\\.\pipe\freeplay_playit_ipc", pub_slot, tunnels_slot, status_slot, logs_slot, broadcaster).await;
                });
            }
        }
        addr
    }

    pub async fn get_tunnels(&self) -> Vec<PlayitTunnelEntry> {
        self.tunnels.read().await.clone()
    }

    pub async fn get_logs(&self) -> Vec<String> {
        self.logs.read().await.iter().cloned().collect()
    }

    pub fn subscribe_logs(&self) -> broadcast::Receiver<String> {
        self.log_broadcaster.subscribe()
    }

    /// Starts the playit-cli agent process and tunnels to local `port`
    pub async fn start_playit_tunnel(&self, _port: u16) -> Result<(), DomainError> {
        let mut proc_lock = self.child.lock().await;
        if let Some(ref mut existing) = *proc_lock {
            if existing.try_wait().ok().flatten().is_none() {
                return Ok(());
            }
        }

        *self.claim_url.write().await = None;
        *self.public_address.write().await = None;

        // 1. Locate or download playit binary
        let binary_path = self.resolve_or_download_binary().await?;

        // 2. Update status to Starting
        *self.status.write().await = PlayitAgentStatus::Starting;
        let init_log = "[playit.gg] Initializing freeplay persistent tunnel...".to_string();
        tracing::info!("{init_log}");
        {
            let mut lg = self.logs.write().await;
            if lg.len() >= 1000 {
                lg.pop_front();
            }
            lg.push_back(init_log.clone());
        }
        let _ = self.log_broadcaster.send(init_log);

        // 3. Spawn persistent daemon process
        let mut cmd = Command::new(&binary_path);
        if let Some(parent) = binary_path.parent() {
            cmd.current_dir(parent);
        }

        let is_cli_wrapper = binary_path
            .file_name()
            .map(|f| f.to_string_lossy().to_lowercase())
            .map(|name| name == "playit.exe" || name == "playit")
            .unwrap_or(false)
            && binary_path.to_string_lossy().contains("playit_gg");

        let tools_dir = dirs::data_local_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("freeplay")
            .join("tools");
        let _ = std::fs::create_dir_all(&tools_dir);
        let secret_file = tools_dir.join("playit.toml");

        // If no secret file exists yet, check if system has a valid configured playit.toml
        if !secret_file.exists() {
            let possible_system_secrets = [
                PathBuf::from(r"C:\ProgramData\playit_gg\playit.toml"),
                dirs::data_local_dir()
                    .unwrap_or_default()
                    .join("playit_gg")
                    .join("playit.toml"),
            ];
            for sys_secret in &possible_system_secrets {
                if sys_secret.exists() {
                    if let Ok(content) = std::fs::read_to_string(sys_secret) {
                        if content.contains("secret_key =") && !content.contains("secret_key = \"\"") {
                            let _ = std::fs::copy(sys_secret, &secret_file);
                            break;
                        }
                    }
                }
            }
        }

        let socket_path_str = if cfg!(windows) {
            r"\\.\pipe\freeplay_playit_ipc".to_string()
        } else {
            std::env::temp_dir()
                .join("freeplay_playit_ipc.sock")
                .to_string_lossy()
                .to_string()
        };

        let system_service_running = if cfg!(windows) {
            std::path::Path::new(r"\\.\pipe\playitd-system").exists()
        } else {
            false
        };

        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW (0x08000000) for headless execution
            cmd.creation_flags(0x08000000);
            if is_cli_wrapper {
                if system_service_running {
                    cmd.arg("attach");
                } else {
                    cmd.arg("-s");
                }
            } else {
                cmd.arg("--socket-path").arg(&socket_path_str);
                cmd.arg("--secret-path").arg(&secret_file);
            }
        }

        #[cfg(not(windows))]
        {
            if is_cli_wrapper {
                if system_service_running {
                    cmd.arg("attach");
                } else {
                    cmd.arg("-s");
                }
            } else {
                cmd.arg("--socket-path").arg(&socket_path_str);
                cmd.arg("--secret-path").arg(&secret_file);
            }
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            DomainError::Internal(format!(
                "Failed to spawn playit agent binary '{:?}': {e}",
                binary_path
            ))
        })?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        *self.stdin.lock().await = stdin;

        let status_clone = self.status.clone();
        let claim_clone = self.claim_url.clone();
        let pub_addr_clone = self.public_address.clone();
        let tunnels_clone = self.tunnels.clone();
        let logs_clone = self.logs.clone();
        let broadcaster = self.log_broadcaster.clone();
        let child_clone = self.child.clone();
        let secret_file_clone = secret_file.clone();
        let socket_for_setup = socket_path_str.clone();

        tokio::spawn(async move {
            let mut tasks = Vec::new();
            if let Some(stdout) = stdout {
                let status_inner = status_clone.clone();
                let claim_inner = claim_clone.clone();
                let pub_addr_inner = pub_addr_clone.clone();
                let tunnels_inner = tunnels_clone.clone();
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();
                let secret_inner = secret_file_clone.clone();
                let socket_setup = socket_for_setup.clone();

                tasks.push(tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        // Detect InvalidAgentKey and clean corrupted/expired secret
                        if line.contains("InvalidAgentKey") || line.contains("configured agent secret is no longer valid") {
                            let _ = std::fs::remove_file(&secret_inner);
                        }

                        // Check for Claim URL
                        if let Some(claim_url) = extract_playit_claim_url(&line) {
                            *claim_inner.write().await = Some(claim_url.clone());
                            *status_inner.write().await = PlayitAgentStatus::Claiming { claim_url };
                        }

                        // Check for Public Domain (e.g. *.gl.joinmc.link, *.playit.gg)
                        if let Some(public_addr) = extract_playit_public_address(&line) {
                            *pub_addr_inner.write().await = Some(public_addr.clone());
                            *status_inner.write().await = PlayitAgentStatus::Connected {
                                public_address: public_addr,
                            };
                        }

                        // If daemon reports tunnels loaded, immediately query tunnel addresses
                        if line.contains("tunnels loaded") || line.contains("playit connected") {
                            let socket_path = socket_setup.clone();
                            let pub_addr_c = pub_addr_inner.clone();
                            let tunnels_c = tunnels_inner.clone();
                            let status_c = status_inner.clone();
                            let logs_c = logs_inner.clone();
                            let broad_c = broadcaster_inner.clone();
                            tokio::spawn(async move {
                                query_playit_tunnels_once(&socket_path, pub_addr_c, tunnels_c, status_c, logs_c, broad_c).await;
                            });
                        }

                        // If daemon is waiting for secret provisioning over IPC, trigger setup
                        if line.contains("Waiting for frontend secret provisioning over IPC") {
                            let socket_path = socket_setup.clone();
                            let claim_c = claim_inner.clone();
                            let status_c = status_inner.clone();
                            let logs_c = logs_inner.clone();
                            let broad_c = broadcaster_inner.clone();
                            tokio::spawn(async move {
                                spawn_playit_setup_helper(&socket_path, claim_c, status_c, logs_c, broad_c).await;
                            });
                        }

                        // Store in logs
                        {
                            let mut lg = logs_inner.write().await;
                            if lg.len() >= 1000 {
                                lg.pop_front();
                            }
                            lg.push_back(line.clone());
                        }

                        let _ = broadcaster_inner.send(line);
                    }
                }));
            }

            if let Some(stderr) = stderr {
                let status_inner = status_clone.clone();
                let claim_inner = claim_clone.clone();
                let pub_addr_inner = pub_addr_clone.clone();
                let tunnels_inner = tunnels_clone.clone();
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();
                let secret_inner = secret_file_clone.clone();
                let socket_setup = socket_for_setup.clone();

                tasks.push(tokio::spawn(async move {
                    let mut reader = BufReader::new(stderr).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        // Detect InvalidAgentKey and clean corrupted/expired secret
                        if line.contains("InvalidAgentKey") || line.contains("configured agent secret is no longer valid") {
                            let _ = std::fs::remove_file(&secret_inner);
                        }

                        // Check for Claim URL
                        if let Some(claim_url) = extract_playit_claim_url(&line) {
                            *claim_inner.write().await = Some(claim_url.clone());
                            *status_inner.write().await = PlayitAgentStatus::Claiming { claim_url };
                        }

                        // Check for Public Domain (e.g. *.gl.joinmc.link, *.playit.gg)
                        if let Some(public_addr) = extract_playit_public_address(&line) {
                            *pub_addr_inner.write().await = Some(public_addr.clone());
                            *status_inner.write().await = PlayitAgentStatus::Connected {
                                public_address: public_addr,
                            };
                        }

                        // If daemon reports tunnels loaded, immediately query tunnel addresses
                        if line.contains("tunnels loaded") || line.contains("playit connected") {
                            let socket_path = socket_setup.clone();
                            let pub_addr_c = pub_addr_inner.clone();
                            let tunnels_c = tunnels_inner.clone();
                            let status_c = status_inner.clone();
                            let logs_c = logs_inner.clone();
                            let broad_c = broadcaster_inner.clone();
                            tokio::spawn(async move {
                                query_playit_tunnels_once(&socket_path, pub_addr_c, tunnels_c, status_c, logs_c, broad_c).await;
                            });
                        }

                        // If daemon is waiting for secret provisioning over IPC, trigger setup
                        if line.contains("Waiting for frontend secret provisioning over IPC") {
                            let socket_path = socket_setup.clone();
                            let claim_c = claim_inner.clone();
                            let status_c = status_inner.clone();
                            let logs_c = logs_inner.clone();
                            let broad_c = broadcaster_inner.clone();
                            tokio::spawn(async move {
                                spawn_playit_setup_helper(&socket_path, claim_c, status_c, logs_c, broad_c).await;
                            });
                        }

                        let formatted_line = format!("[STDERR] {}", line);
                        {
                            let mut lg = logs_inner.write().await;
                            if lg.len() >= 1000 {
                                lg.pop_front();
                            }
                            lg.push_back(formatted_line.clone());
                        }

                        let _ = broadcaster_inner.send(formatted_line);
                    }
                }));
            }

            // Background active tunnel resolver to query tunnel status from daemon
            let pub_addr_resolver = pub_addr_clone.clone();
            let tunnels_resolver = tunnels_clone.clone();
            let status_resolver = status_clone.clone();
            let logs_resolver = logs_clone.clone();
            let broadcaster_resolver = broadcaster.clone();
            let socket_resolver = socket_for_setup.clone();

            tasks.push(tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                for _ in 0..60 {
                    if pub_addr_resolver.read().await.is_some() {
                        break;
                    }

                    query_playit_tunnels_once(
                        &socket_resolver,
                        pub_addr_resolver.clone(),
                        tunnels_resolver.clone(),
                        status_resolver.clone(),
                        logs_resolver.clone(),
                        broadcaster_resolver.clone(),
                    )
                    .await;

                    if pub_addr_resolver.read().await.is_some() {
                        break;
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }));

            for t in tasks {
                let _ = t.await;
            }

            *status_clone.write().await = PlayitAgentStatus::Stopped;
            *child_clone.lock().await = None;
        });

        *proc_lock = Some(child);
        Ok(())
    }

    /// Stops the playit tunnel process gracefully by sending terminal 'q' followed by 'y',
    /// closing the online IP and clearing active tunnels.
    pub async fn stop_playit_tunnel(&self) -> Result<(), DomainError> {
        // 1. Send preset terminal input: 'q\n' then 'y\n' via stdin
        let mut stdin_lock = self.stdin.lock().await;
        if let Some(mut stdin) = stdin_lock.take() {
            use tokio::io::AsyncWriteExt;
            let _ = stdin.write_all(b"q\n").await;
            let _ = stdin.flush().await;

            // Brief delay to allow playit interactive confirmation prompt
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

            let _ = stdin.write_all(b"y\n").await;
            let _ = stdin.flush().await;
        }

        // 2. Wait up to 2 seconds for graceful process shutdown
        let mut proc_lock = self.child.lock().await;
        if let Some(mut child) = proc_lock.take() {
            let wait_res = tokio::time::timeout(
                tokio::time::Duration::from_millis(2000),
                child.wait(),
            )
            .await;

            if wait_res.is_err() {
                let _ = child.kill().await;
            }
        }

        // 3. On Windows, ensure any detached playit.exe daemon processes are terminated
        #[cfg(windows)]
        {
            let _ = tokio::process::Command::new("taskkill")
                .args(["/F", "/IM", "playit.exe", "/T"])
                .creation_flags(0x08000000)
                .output()
                .await;
        }

        // 4. Immediately clear online IP, active tunnels, and claim URL
        *self.status.write().await = PlayitAgentStatus::Stopped;
        *self.claim_url.write().await = None;
        *self.public_address.write().await = None;
        self.tunnels.write().await.clear();

        let stop_log = "[playit.gg] Tunnel disconnected. Online IP closed.".to_string();
        {
            let mut lg = self.logs.write().await;
            if lg.len() >= 1000 {
                lg.pop_front();
            }
            lg.push_back(stop_log.clone());
        }
        let _ = self.log_broadcaster.send(stop_log);

        Ok(())
    }

    async fn resolve_or_download_binary(&self) -> Result<PathBuf, DomainError> {
        // 1. Check custom path if configured
        if let Some(ref path) = *self.binary_path.read().await {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        let exe_name = if cfg!(windows) { "playit.exe" } else { "playit" };
        let daemon_name = if cfg!(windows) { "playitd.exe" } else { "playitd" };

        // 2. Check local tools directory cache
        let tools_dir = dirs::data_local_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("freeplay")
            .join("tools");
        for name in &[daemon_name, exe_name] {
            let tools_bin = tools_dir.join(name);
            if tools_bin.exists() {
                return Ok(tools_bin);
            }
        }

        // 3. Check ancestor directories of current_dir and current_exe (up to 6 levels)
        let mut check_dirs = Vec::new();
        if let Ok(cd) = std::env::current_dir() {
            check_dirs.push(cd);
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                check_dirs.push(parent.to_path_buf());
            }
        }

        for base in check_dirs {
            let mut curr = Some(base.as_path());
            for _ in 0..6 {
                if let Some(dir) = curr {
                    for name in &[daemon_name, exe_name] {
                        let candidate = dir.join("bin").join("playit").join(name);
                        if candidate.exists() {
                            return Ok(candidate);
                        }
                        let candidate2 = dir.join(name);
                        if candidate2.exists() {
                            return Ok(candidate2);
                        }
                    }
                    curr = dir.parent();
                } else {
                    break;
                }
            }
        }

        // 4. Check system PATH — prefer playitd.exe over playit.exe if present
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let daemon_p = dir.join(daemon_name);
                if daemon_p.exists() {
                    return Ok(daemon_p);
                }
                let exe_p = dir.join(exe_name);
                if exe_p.exists() {
                    // If playitd.exe is in the same directory as playit.exe, prefer playitd.exe
                    if let Some(p) = exe_p.parent() {
                        let sibling_daemon = p.join(daemon_name);
                        if sibling_daemon.exists() {
                            return Ok(sibling_daemon);
                        }
                    }
                    return Ok(exe_p);
                }
            }
        }

        // 5. Download standalone playit binary to tools cache
        *self.status.write().await = PlayitAgentStatus::Downloading;
        download_playit_binary(&tools_dir).await
    }
}

// =========================================================================
// Configuration & Parsing Helpers
// =========================================================================

/// Automatically configures server.properties ensuring:
/// - `online-mode=false`
/// - `enforce-secure-profile=false`
/// - `server-port={port}`
pub fn configure_server_properties(server_dir: &Path, port: u16) -> Result<(), std::io::Error> {
    let prop_file = server_dir.join("server.properties");
    if prop_file.exists() {
        let content = std::fs::read_to_string(&prop_file)?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        let mut found_online_mode = false;
        let mut found_secure_profile = false;
        let mut found_port = false;

        for line in &mut lines {
            let trimmed = line.trim();
            if trimmed.starts_with("online-mode") {
                *line = "online-mode=false".to_string();
                found_online_mode = true;
            } else if trimmed.starts_with("enforce-secure-profile") {
                *line = "enforce-secure-profile=false".to_string();
                found_secure_profile = true;
            } else if trimmed.starts_with("server-port") {
                *line = format!("server-port={}", port);
                found_port = true;
            }
        }

        if !found_online_mode {
            lines.push("online-mode=false".to_string());
        }
        if !found_secure_profile {
            lines.push("enforce-secure-profile=false".to_string());
        }
        if !found_port {
            lines.push(format!("server-port={}", port));
        }

        std::fs::write(&prop_file, lines.join("\n") + "\n")?;
    } else {
        let default_properties = format!(
            "#Minecraft server properties\n\
             #Generated by FreePlay Launcher\n\
             enable-jmx-monitoring=false\n\
             rcon.port=25575\n\
             level-seed=\n\
             gamemode=survival\n\
             enable-command-block=true\n\
             enable-query=false\n\
             generator-settings={{}}\n\
             enforce-secure-profile=false\n\
             level-name=world\n\
             motd=A FreePlay Minecraft Server\n\
             query.port={port}\n\
             pvp=true\n\
             generate-structures=true\n\
             difficulty=easy\n\
             network-compression-threshold=256\n\
             require-resource-pack=false\n\
             max-tick-time=60000\n\
             use-native-transport=true\n\
             max-players=20\n\
             online-mode=false\n\
             enable-status=true\n\
             allow-flight=true\n\
             broadcast-rcon-to-ops=true\n\
             view-distance=10\n\
             server-ip=\n\
             resource-pack-prompt=\n\
             allow-nether=true\n\
             server-port={port}\n\
             enable-rcon=false\n\
             sync-chunk-writes=true\n\
             op-permission-level=4\n\
             prevent-proxy-connections=false\n\
             hide-online-players=false\n\
             resource-pack=\n\
             entity-broadcast-range-percentage=100\n\
             simulation-distance=10\n\
             rcon.password=\n\
             player-idle-timeout=0\n\
             force-gamemode=false\n\
             rate-limit=0\n\
             hardcore=false\n\
             white-list=false\n\
             broadcast-console-to-ops=true\n\
             spawn-npcs=true\n\
             spawn-animals=true\n\
             function-permission-level=2\n\
             initial-enabled-packs=vanilla\n\
             level-type=minecraft\\:normal\n\
             text-filtering-config=\n\
             spawn-monsters=true\n\
             enforce-whitelist=false\n\
             spawn-protection=0\n\
             resource-pack-sha1=\n\
             max-world-size=29999984\n"
        );
        std::fs::write(&prop_file, default_properties)?;
    }
    Ok(())
}

/// Automatically creates eula.txt with eula=true
pub fn ensure_eula_accepted(server_dir: &Path) -> Result<(), std::io::Error> {
    let eula_file = server_dir.join("eula.txt");
    std::fs::write(
        &eula_file,
        "#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).\neula=true\n",
    )
}

pub fn resolve_playit_cli_path() -> Option<PathBuf> {
    let exe_name = if cfg!(windows) { "playit.exe" } else { "playit" };

    // 1. Check system PATH
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join(exe_name);
            if p.exists() {
                return Some(p);
            }
        }
    }

    // 2. Check standard install locations
    let candidates = [
        PathBuf::from(r"C:\Program Files\playit_gg\bin\playit.exe"),
        PathBuf::from(r"C:\Program Files\playit_gg\playit.exe"),
        dirs::data_local_dir()
            .unwrap_or_default()
            .join("freeplay")
            .join("tools")
            .join(exe_name),
    ];
    for cand in &candidates {
        if cand.exists() {
            return Some(cand.clone());
        }
    }

    None
}

pub fn strip_ansi_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c.is_ascii_alphabetic() || c == 'm' || c == 'H' || c == 'J' || c == 'K' || c == 'h' || c == 'l' {
                in_escape = false;
                out.push(' ');
            }
        } else if c == '\r' || c == '\n' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

/// Extracts a playit.gg claim URL from log lines
pub fn extract_playit_claim_url(line: &str) -> Option<String> {
    let clean_line = strip_ansi_escapes(line);
    if let Some(idx) = clean_line.find("https://playit.gg/claim/") {
        let after = &clean_line[idx..];
        let end = after
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ')' || c == ']' || c == '!')
            .unwrap_or(after.len());
        return Some(after[..end].to_string());
    }
    None
}

/// Extracts a playit public domain/address from log lines (e.g. *.gl.joinmc.link, *.playit.gg, *.joinmc.net, or IP:port)
pub fn extract_playit_public_address(line: &str) -> Option<String> {
    let clean_line = strip_ansi_escapes(line);

    // Check for mapping lines from `playit attach`: "● simmons-scanners.tun.ply.gg => 127.0.0.1:25565"
    if let Some((left, _)) = clean_line.split_once("=>") {
        for token in left.split_whitespace() {
            let clean = token.trim_matches(|c: char| {
                c == '│' || c == '●' || c == ' ' || c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == '>' || c == '<' || c == '='
            });
            if clean.contains(".ply.gg") || clean.contains(".joinmc.link") || clean.contains(".playit.gg") || clean.contains(".joinmc.net") || clean.contains(".playit.link") {
                let addr = clean.strip_prefix("tcp://").unwrap_or(clean);
                let addr = addr.strip_prefix("udp://").unwrap_or(addr);
                return Some(addr.to_string());
            }
        }
    }

    // Check for known playit domain names in whitespace-separated tokens
    for token in clean_line.split_whitespace() {
        let clean = token.trim_matches(|c: char| {
            c == '│' || c == '●' || c == ' ' || c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == '>' || c == '<' || c == '='
        });
        if clean.contains(".joinmc.link") || clean.contains(".playit.gg") || clean.contains(".joinmc.net") || clean.contains(".playit.link") || clean.contains(".ply.gg") {
            let addr = clean.strip_prefix("tcp://").unwrap_or(clean);
            let addr = addr.strip_prefix("udp://").unwrap_or(addr);
            return Some(addr.to_string());
        }
    }

    // Check for "address=HOST:PORT" or "addr=HOST:PORT" key-value patterns
    let lower = clean_line.to_lowercase();
    for pattern in &["address=", "addr=", "public_address=", "tunnel_address="] {
        if let Some(idx) = lower.find(pattern) {
            let after = &clean_line[idx + pattern.len()..];
            let end = after
                .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == ']' || c == '}')
                .unwrap_or(after.len());
            let candidate = &after[..end];
            if candidate.contains(':') && !candidate.is_empty() {
                return Some(candidate.to_string());
            }
        }
    }

    // Check for raw IP:port pattern where the line mentions "tunnel" context
    if lower.contains("tunnel") || lower.contains("ready") || lower.contains("listening") || lower.contains("forwarding") {
        for token in clean_line.split_whitespace() {
            let clean = token.trim_matches(|c: char| {
                c == '│' || c == '●' || c == ' ' || c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == '>' || c == '<' || c == '='
            });
            if let Some((host, port_str)) = clean.rsplit_once(':') {
                if port_str.parse::<u16>().is_ok()
                    && !host.is_empty()
                    && host != "127.0.0.1"
                    && host != "0.0.0.0"
                    && host != "localhost"
                    && (host.contains('.') || host.contains(':'))
                {
                    return Some(clean.to_string());
                }
            }
        }
    }

    None
}

/// Extracts all active playit public tunnels (Java, Bedrock, Voice, Map, etc.) from text
pub fn extract_all_playit_tunnels(raw_text: &str) -> Vec<PlayitTunnelEntry> {
    let clean_line = strip_ansi_escapes(raw_text);
    let mut tunnels = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let words: Vec<&str> = clean_line.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if *word == "=>" && i > 0 && i + 1 < words.len() {
            let left = words[i - 1].trim_matches(|c: char| {
                c == '│' || c == '●' || c == ' ' || c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == '>' || c == '<' || c == '='
            });
            let right = words[i + 1].trim_matches(|c: char| {
                c == '│' || c == '●' || c == ' ' || c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == '>' || c == '<' || c == '='
            });

            if (left.contains(".ply.gg") || left.contains(".joinmc.link") || left.contains(".playit.gg") || left.contains(".joinmc.net") || left.contains(".playit.link"))
                && !seen.contains(left)
            {
                seen.insert(left.to_string());
                let tunnel_type = if right.ends_with(":25565") || right == "25565" {
                    "Minecraft Java Edition".to_string()
                } else if right.ends_with(":19132") || right == "19132" {
                    "Minecraft Bedrock (Geyser)".to_string()
                } else if right.ends_with(":24454") || right == "24454" {
                    "Simple Voice Chat".to_string()
                } else if right.ends_with(":8123") || right == "8123" {
                    "Dynmap Web".to_string()
                } else if right.ends_with(":8100") || right == "8100" {
                    "BlueMap Web".to_string()
                } else {
                    format!("Local Port {}", right.rsplit_once(':').map(|(_, p)| p).unwrap_or(right))
                };

                tunnels.push(PlayitTunnelEntry {
                    domain: left.to_string(),
                    target: right.to_string(),
                    tunnel_type,
                });
            }
        }
    }

    tunnels
}

/// Queries the running playit daemon via `playit attach` over IPC using raw chunk reading to parse active tunnels
pub async fn query_playit_tunnels_once(
    socket_path: &str,
    pub_addr_slot: Arc<RwLock<Option<String>>>,
    tunnels_slot: Arc<RwLock<Vec<PlayitTunnelEntry>>>,
    status_slot: Arc<RwLock<PlayitAgentStatus>>,
    logs_slot: Arc<RwLock<VecDeque<String>>>,
    broadcaster: broadcast::Sender<String>,
) {
    let cli_path = resolve_playit_cli_path().unwrap_or_else(|| PathBuf::from(if cfg!(windows) { "playit.exe" } else { "playit" }));

    let candidate_sockets = [
        Some(socket_path.to_string()),
        if cfg!(windows) {
            Some(r"\\.\pipe\playitd-system".to_string())
        } else {
            None
        },
        None,
    ];

    for sock_opt in candidate_sockets.into_iter().flatten() {
        let mut attach_cmd = Command::new(&cli_path);
        #[cfg(windows)]
        {
            attach_cmd.creation_flags(0x08000000);
        }
        if !sock_opt.is_empty() {
            attach_cmd.arg("--socket-path").arg(&sock_opt);
        }
        attach_cmd.arg("attach");
        attach_cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        if let Ok(mut child) = attach_cmd.spawn() {
            if let Some(mut stdout) = child.stdout.take() {
                let mut buffer = [0u8; 8192];
                let mut accumulated = Vec::new();

                let start_time = tokio::time::Instant::now();
                while start_time.elapsed() < tokio::time::Duration::from_millis(1500) {
                    match tokio::time::timeout(tokio::time::Duration::from_millis(300), stdout.read(&mut buffer)).await {
                        Ok(Ok(n)) if n > 0 => {
                            accumulated.extend_from_slice(&buffer[..n]);
                            let text = String::from_utf8_lossy(&accumulated);
                            let all_tunnels = extract_all_playit_tunnels(&text);
                            if !all_tunnels.is_empty() {
                                let primary = all_tunnels.iter()
                                    .find(|t| t.tunnel_type.contains("Java") || t.target.ends_with(":25565"))
                                    .unwrap_or(&all_tunnels[0])
                                    .domain
                                    .clone();

                                *pub_addr_slot.write().await = Some(primary.clone());
                                *tunnels_slot.write().await = all_tunnels.clone();
                                *status_slot.write().await = PlayitAgentStatus::Connected {
                                    public_address: primary.clone(),
                                };
                                let msg = format!("[playit] Connected {} Active Tunnel(s). Primary: {primary}", all_tunnels.len());
                                {
                                    let mut lg = logs_slot.write().await;
                                    lg.push_back(msg.clone());
                                }
                                let _ = broadcaster.send(msg);
                                let _ = child.kill().await;
                                return;
                            } else if let Some(public_addr) = extract_playit_public_address(&text) {
                                *pub_addr_slot.write().await = Some(public_addr.clone());
                                *tunnels_slot.write().await = vec![PlayitTunnelEntry {
                                    domain: public_addr.clone(),
                                    target: "127.0.0.1:25565".to_string(),
                                    tunnel_type: "Minecraft Java Edition".to_string(),
                                }];
                                *status_slot.write().await = PlayitAgentStatus::Connected {
                                    public_address: public_addr.clone(),
                                };
                                let msg = format!("[playit] Active Tunnel Connected: {public_addr}");
                                {
                                    let mut lg = logs_slot.write().await;
                                    lg.push_back(msg.clone());
                                }
                                let _ = broadcaster.send(msg);
                                let _ = child.kill().await;
                                return;
                            }
                        }
                        _ => break,
                    }
                }
                let _ = child.kill().await;
            }
        }
    }
}

/// Spawns the playit setup tool to connect over IPC, obtain the claim URL, and provision credentials
pub async fn spawn_playit_setup_helper(
    socket_path: &str,
    claim_slot: Arc<RwLock<Option<String>>>,
    status_slot: Arc<RwLock<PlayitAgentStatus>>,
    logs_slot: Arc<RwLock<VecDeque<String>>>,
    broadcaster: broadcast::Sender<String>,
) {
    // 1. Locate the playit CLI controller executable
    let mut cli_path: Option<PathBuf> = None;
    let exe_name = if cfg!(windows) { "playit.exe" } else { "playit" };

    // Check system PATH
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join(exe_name);
            if p.exists() {
                cli_path = Some(p);
                break;
            }
        }
    }

    // Check known locations
    if cli_path.is_none() {
        let candidates = [
            PathBuf::from(r"C:\Program Files\playit_gg\bin\playit.exe"),
            dirs::data_local_dir()
                .unwrap_or_default()
                .join("freeplay")
                .join("tools")
                .join(exe_name),
        ];
        for cand in &candidates {
            if cand.exists() {
                cli_path = Some(cand.clone());
                break;
            }
        }
    }

    if let Some(cli) = cli_path {
        let mut cmd = Command::new(&cli);
        #[cfg(windows)]
        {
            cmd.creation_flags(0x08000000);
            cmd.arg("--socket-path").arg(socket_path).arg("setup");
        }
        #[cfg(not(windows))]
        {
            cmd.arg("--socket-path").arg(socket_path).arg("setup");
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        if let Ok(mut child) = cmd.spawn() {
            let stdout = child.stdout.take();
            if let Some(stdout) = stdout {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    if let Some(claim_url) = extract_playit_claim_url(&line) {
                        *claim_slot.write().await = Some(claim_url.clone());
                        *status_slot.write().await = PlayitAgentStatus::Claiming {
                            claim_url: claim_url.clone(),
                        };
                    }

                    {
                        let mut lg = logs_slot.write().await;
                        if lg.len() >= 1000 {
                            lg.pop_front();
                        }
                        lg.push_back(format!("[playit-setup] {}", line));
                    }
                    let _ = broadcaster.send(format!("[playit-setup] {}", line));
                }
            }
            let _ = child.wait().await;
        }
    }
}

/// Builds optimized JVM arguments for the Minecraft server process based on selected preset and memory allocation
pub fn build_server_jvm_args(
    ram_mb: u32,
    preset: &str,
    custom_args: Option<&str>,
) -> Vec<String> {
    let ram = if ram_mb < 512 { 2048 } else { ram_mb };
    let mut args = Vec::new();

    // Memory Heap Allocation
    args.push(format!("-Xms{}M", ram));
    args.push(format!("-Xmx{}M", ram));

    match preset.to_lowercase().as_str() {
        "aikar" | "aikars" => {
            // Aikar's High-Performance Flags (Gold standard for Minecraft servers)
            args.extend([
                "-XX:+UseG1GC".to_string(),
                "-XX:+ParallelRefProcEnabled".to_string(),
                "-XX:MaxGCPauseMillis=200".to_string(),
                "-XX:+UnlockExperimentalVMOptions".to_string(),
                "-XX:+DisableExplicitGC".to_string(),
                "-XX:+AlwaysPreTouch".to_string(),
                "-XX:G1NewSizePercent=30".to_string(),
                "-XX:G1MaxNewSizePercent=40".to_string(),
                "-XX:G1ReservePercent=20".to_string(),
                "-XX:G1HeapWastePercent=5".to_string(),
                "-XX:G1MixedGCCountTarget=4".to_string(),
                "-XX:InitiatingHeapOccupancyPercent=15".to_string(),
                "-XX:G1MixedGCLiveThresholdPercent=90".to_string(),
                "-XX:G1RSetUpdatingPauseTimePercent=5".to_string(),
                "-XX:SurvivorRatio=32".to_string(),
                "-XX:+PerfDisableSharedMem".to_string(),
                "-XX:MaxTenuringThreshold=1".to_string(),
                "-Dusing.aikars.flags=https://mcflags.emc.gs".to_string(),
                "-Daikars.new.flags=true".to_string(),
            ]);
        }
        "zgc" => {
            // Generational ZGC (Ultra-Low Latency for Java 21+)
            args.extend([
                "-XX:+UseZGC".to_string(),
                "-XX:+ZGenerational".to_string(),
                "-XX:+AlwaysPreTouch".to_string(),
                "-XX:+DisableExplicitGC".to_string(),
            ]);
        }
        "shenandoah" => {
            // Shenandoah Ultra-Low Pause Concurrent GC
            args.extend([
                "-XX:+UseShenandoahGC".to_string(),
                "-XX:ShenandoahGCMode=iu".to_string(),
                "-XX:+AlwaysPreTouch".to_string(),
                "-XX:+DisableExplicitGC".to_string(),
            ]);
        }
        "custom" => {
            if let Some(custom) = custom_args {
                for token in custom.split_whitespace() {
                    if !token.is_empty() {
                        args.push(token.to_string());
                    }
                }
            }
        }
        _ => {
            // Balanced / Standard Default
            args.extend([
                "-XX:+UseG1GC".to_string(),
                "-XX:+AlwaysPreTouch".to_string(),
            ]);
        }
    }

    args
}

/// Finds an installed Java binary on the system
pub fn find_java_binary() -> PathBuf {
    // 1. Check JAVA_HOME
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let p = Path::new(&java_home)
            .join("bin")
            .join(if cfg!(windows) { "java.exe" } else { "java" });
        if p.exists() {
            return p;
        }
    }

    // 2. Check PATH
    if let Ok(path_var) = std::env::var("PATH") {
        let exe = if cfg!(windows) { "java.exe" } else { "java" };
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join(exe);
            if p.exists() {
                return p;
            }
        }
    }

    // 3. Check common Windows paths
    #[cfg(windows)]
    {
        let roots = [
            r"C:\Program Files\Java",
            r"C:\Program Files\Eclipse Adoptium",
            r"C:\Program Files\Microsoft",
            r"C:\Program Files\BellSoft",
            r"C:\Program Files\Amazon Corretto",
            r"C:\Program Files\Zulu",
        ];

        for root in roots {
            let root_path = Path::new(root);
            if root_path.exists() {
                if let Ok(entries) = std::fs::read_dir(root_path) {
                    for entry in entries.flatten() {
                        let java_exe = entry.path().join("bin").join("java.exe");
                        if java_exe.exists() {
                            return java_exe;
                        }
                    }
                }
            }
        }
    }

    // Fallback
    PathBuf::from(if cfg!(windows) { "java.exe" } else { "java" })
}

/// Locates or downloads the server JAR for PaperMC, Fabric, or Vanilla
async fn locate_or_download_server_jar(
    version: &str,
    server_type: &str,
    working_dir: &Path,
) -> Result<PathBuf, DomainError> {
    // Check existing JARs in working directory
    let server_jar = working_dir.join("server.jar");
    if server_jar.exists() {
        return Ok(server_jar);
    }

    let type_jar = working_dir.join(format!("{}-{}.jar", server_type.to_lowercase(), version));
    if type_jar.exists() {
        return Ok(type_jar);
    }

    download_server_jar(version, server_type, &server_jar).await
}

/// Downloads a dedicated server JAR (PaperMC, Fabric, Vanilla)
pub async fn download_server_jar(
    version: &str,
    server_type: &str,
    target_path: &Path,
) -> Result<PathBuf, DomainError> {
    let client = reqwest::Client::builder()
        .user_agent("FreePlay-Minecraft-Launcher/0.1.0")
        .build()
        .map_err(|e| DomainError::Internal(format!("Failed to build HTTP client: {e}")))?;

    let server_type_lower = server_type.to_lowercase();
    let download_url = match server_type_lower.as_str() {
        "paper" | "papermc" => {
            // PaperMC Fill v3 API
            let version_url = format!("https://fill.papermc.io/v3/projects/paper/versions/{}", version);
            let resp = client
                .get(&version_url)
                .send()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to query PaperMC API: {e}")))?;

            if !resp.status().is_success() {
                return Err(DomainError::Internal(format!(
                    "PaperMC API error for version {}: HTTP {}",
                    version,
                    resp.status()
                )));
            }

            let data: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse PaperMC response: {e}")))?;

            let latest_build = data["builds"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    DomainError::Internal(format!("No PaperMC builds found for version {}", version))
                })?;

            let build_url = format!(
                "https://fill.papermc.io/v3/projects/paper/versions/{}/builds/{}",
                version, latest_build
            );
            let build_resp = client
                .get(&build_url)
                .send()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to query PaperMC build API: {e}")))?;

            let build_data: serde_json::Value = build_resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse PaperMC build response: {e}")))?;

            let mut direct_url = None;
            if let Some(downloads) = build_data["downloads"].as_object() {
                for (_key, val) in downloads {
                    if let Some(url) = val["url"].as_str() {
                        direct_url = Some(url.to_string());
                        break;
                    }
                }
            }

            direct_url.ok_or_else(|| {
                DomainError::Internal(format!(
                    "Could not resolve PaperMC download URL for version {} build {}",
                    version, latest_build
                ))
            })?
        }
        "fabric" => {
            // Fabric Meta API
            let loader_url = format!("https://meta.fabricmc.net/v2/versions/loader/{}", version);
            let resp = client
                .get(&loader_url)
                .send()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to query Fabric Meta API: {e}")))?;

            let loader_ver = if resp.status().is_success() {
                let arr: serde_json::Value = resp.json().await.unwrap_or_default();
                arr.as_array()
                    .and_then(|a| a.first())
                    .and_then(|obj| obj["loader"]["version"].as_str())
                    .unwrap_or("0.16.10")
                    .to_string()
            } else {
                "0.16.10".to_string()
            };

            format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/{}/1.0.1/server/jar",
                version, loader_ver
            )
        }
        _ => {
            // Vanilla Mojang API
            let manifest_url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
            let resp = client
                .get(manifest_url)
                .send()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to fetch Mojang manifest: {e}")))?;

            let manifest: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse Mojang manifest: {e}")))?;

            let version_entry = manifest["versions"]
                .as_array()
                .and_then(|versions| {
                    versions.iter().find(|v| v["id"].as_str() == Some(version))
                })
                .ok_or_else(|| {
                    DomainError::Internal(format!("Version '{}' not found in Mojang manifest", version))
                })?;

            let version_meta_url = version_entry["url"].as_str().ok_or_else(|| {
                DomainError::Internal("Missing URL for version meta".to_string())
            })?;

            let meta_resp = client
                .get(version_meta_url)
                .send()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to fetch version metadata: {e}")))?;

            let meta_json: serde_json::Value = meta_resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse version metadata: {e}")))?;

            meta_json["downloads"]["server"]["url"]
                .as_str()
                .ok_or_else(|| {
                    DomainError::Internal(format!(
                        "No dedicated server download available for Vanilla {}",
                        version
                    ))
                })?
                .to_string()
        }
    };

    // Download JAR bytes to target_path
    let jar_bytes = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| DomainError::Internal(format!("Failed to download server JAR from {download_url}: {e}")))?
        .bytes()
        .await
        .map_err(|e| DomainError::Internal(format!("Failed to read server JAR bytes: {e}")))?;

    if let Some(parent) = target_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            DomainError::Internal(format!("Failed to create parent directory: {e}"))
        })?;
    }

    tokio::fs::write(target_path, &jar_bytes).await.map_err(|e| {
        DomainError::Internal(format!("Failed to write server JAR to {:?}: {e}", target_path))
    })?;

    Ok(target_path.to_path_buf())
}

/// Downloads official playit agent binary to `target_dir`
pub async fn download_playit_binary(target_dir: &Path) -> Result<PathBuf, DomainError> {
    tokio::fs::create_dir_all(target_dir).await.map_err(|e| {
        DomainError::Internal(format!("Failed to create playit tools directory: {e}"))
    })?;

    let exe_name = if cfg!(windows) { "playit.exe" } else { "playit" };
    let target_path = target_dir.join(exe_name);

    let client = reqwest::Client::builder()
        .user_agent("FreePlay-Minecraft-Launcher/0.1.0")
        .build()
        .map_err(|e| DomainError::Internal(format!("Failed to build HTTP client: {e}")))?;

    #[cfg(target_os = "windows")]
    let download_url = "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-windows-x86_64.exe";

    #[cfg(target_os = "linux")]
    let download_url = "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-linux-x86_64";

    #[cfg(target_os = "macos")]
    let download_url = "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-darwin-x86_64";

    let resp = client
        .get(download_url)
        .send()
        .await
        .map_err(|e| DomainError::Internal(format!("Failed to download playit binary from {download_url}: {e}")))?;

    if !resp.status().is_success() {
        return Err(DomainError::Internal(format!(
            "Failed to download playit agent: HTTP {}",
            resp.status()
        )));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| DomainError::Internal(format!("Failed to read playit bytes: {e}")))?;

    tokio::fs::write(&target_path, &bytes).await.map_err(|e| {
        DomainError::Internal(format!("Failed to write playit binary to {:?}: {e}", target_path))
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&target_path)
            .map_err(|e| DomainError::Internal(format!("Failed to get permissions: {e}")))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&target_path, perms)
            .map_err(|e| DomainError::Internal(format!("Failed to set exec permission: {e}")))?;
    }

    Ok(target_path)
}

// =========================================================================
// Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_server_properties_new_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let port = 25568;

        configure_server_properties(temp_dir.path(), port).unwrap();

        let prop_path = temp_dir.path().join("server.properties");
        assert!(prop_path.exists());

        let content = std::fs::read_to_string(prop_path).unwrap();
        assert!(content.contains("online-mode=false"));
        assert!(content.contains("enforce-secure-profile=false"));
        assert!(content.contains("server-port=25568"));
    }

    #[test]
    fn test_configure_server_properties_existing_file_update() {
        let temp_dir = tempfile::tempdir().unwrap();
        let prop_path = temp_dir.path().join("server.properties");

        // Existing file with online-mode=true and enforce-secure-profile=true
        std::fs::write(
            &prop_path,
            "motd=Custom MOTD\nonline-mode=true\nenforce-secure-profile=true\nserver-port=25565\nmax-players=50\n",
        )
        .unwrap();

        configure_server_properties(temp_dir.path(), 25570).unwrap();

        let updated_content = std::fs::read_to_string(prop_path).unwrap();
        assert!(updated_content.contains("online-mode=false"));
        assert!(!updated_content.contains("online-mode=true"));
        assert!(updated_content.contains("enforce-secure-profile=false"));
        assert!(!updated_content.contains("enforce-secure-profile=true"));
        assert!(updated_content.contains("server-port=25570"));
        assert!(updated_content.contains("motd=Custom MOTD"));
        assert!(updated_content.contains("max-players=50"));
    }

    #[test]
    fn test_ensure_eula_accepted() {
        let temp_dir = tempfile::tempdir().unwrap();
        ensure_eula_accepted(temp_dir.path()).unwrap();

        let eula_path = temp_dir.path().join("eula.txt");
        assert!(eula_path.exists());

        let content = std::fs::read_to_string(eula_path).unwrap();
        assert!(content.contains("eula=true"));
    }

    #[test]
    fn test_extract_playit_claim_url() {
        let line = "Claim your agent at: https://playit.gg/claim/abc123xyz !";
        let claim = extract_playit_claim_url(line);
        assert_eq!(claim, Some("https://playit.gg/claim/abc123xyz".to_string()));

        let line2 = "No claim in this line";
        assert_eq!(extract_playit_claim_url(line2), None);
    }

    #[test]
    fn test_extract_playit_public_address() {
        let line1 = "Tunnel active: epic-minecraft.gl.joinmc.link:25565 => 127.0.0.1:25565";
        let addr1 = extract_playit_public_address(line1);
        assert_eq!(addr1, Some("epic-minecraft.gl.joinmc.link:25565".to_string()));

        let line2 = "Connected to server.playit.gg:12345";
        let addr2 = extract_playit_public_address(line2);
        assert_eq!(addr2, Some("server.playit.gg:12345".to_string()));

        let line3 = "│● simmons-scanners.tun.ply.gg => 127.0.0.1:25565                                                                      │";
        let addr3 = extract_playit_public_address(line3);
        assert_eq!(addr3, Some("simmons-scanners.tun.ply.gg".to_string()));

        let line4 = "\x1b[?1049h\x1b[5;1H│\x1b[38;5;2;49m● simmons-scanners.tun.ply.gg => 127.0.0.1:25565\x1b[38;5;6;49m│\x1b[6;1H│● simmons-lang.tun.ply.gg:25631 => 127.0.0.1:19132";
        let addr4 = extract_playit_public_address(line4);
        assert_eq!(addr4, Some("simmons-scanners.tun.ply.gg".to_string()));

        let line5 = "Random log output from server";
        assert_eq!(extract_playit_public_address(line5), None);
    }

    #[test]
    fn test_extract_all_playit_tunnels() {
        let stream = "\x1b[?1049h\x1b[5;1H│\x1b[38;5;2;49m● simmons-scanners.tun.ply.gg => 127.0.0.1:25565\x1b[38;5;6;49m│\x1b[6;1H│● simmons-lang.tun.ply.gg:25631 => 127.0.0.1:19132";
        let tunnels = extract_all_playit_tunnels(stream);
        assert_eq!(tunnels.len(), 2);
        assert_eq!(tunnels[0].domain, "simmons-scanners.tun.ply.gg");
        assert_eq!(tunnels[0].target, "127.0.0.1:25565");
        assert_eq!(tunnels[0].tunnel_type, "Minecraft Java Edition");

        assert_eq!(tunnels[1].domain, "simmons-lang.tun.ply.gg:25631");
        assert_eq!(tunnels[1].target, "127.0.0.1:19132");
        assert_eq!(tunnels[1].tunnel_type, "Minecraft Bedrock (Geyser)");
    }

    #[tokio::test]
    async fn test_server_supervisor_initial_state() {
        let supervisor = ServerProcessSupervisor::new();
        assert_eq!(supervisor.get_status().await, DedicatedServerStatus::Stopped);
        assert!(!supervisor.is_running().await);
    }

    #[tokio::test]
    async fn test_playit_supervisor_initial_state() {
        let supervisor = PlayitTunnelSupervisor::new();
        assert_eq!(supervisor.get_status().await, PlayitAgentStatus::Stopped);
        assert_eq!(supervisor.get_claim_url().await, None);
        assert_eq!(supervisor.get_public_address().await, None);
    }

    #[test]
    fn test_parse_player_join_log() {
        let line1 = "[12:34:56 INFO]: Gautam joined the game";
        let res1 = parse_player_join_log(line1);
        assert_eq!(res1, Some(("Gautam".to_string(), None, None)));

        let line2 = "[12:34:56 INFO]: Gautam[/127.0.0.1:54321] logged in with entity id 123 at ([world]0.0, 80.0, 0.0)";
        let res2 = parse_player_join_log(line2);
        assert_eq!(res2, Some(("Gautam".to_string(), None, Some("127.0.0.1".to_string()))));
    }

    #[test]
    fn test_parse_player_uuid_log() {
        let line = "[12:34:56 INFO]: UUID of player Gautam is a01e3843-e521-3998-958a-f459800e4d11";
        let res = parse_player_uuid_log(line);
        assert_eq!(res, Some(("Gautam".to_string(), "a01e3843-e521-3998-958a-f459800e4d11".to_string())));
    }

    #[test]
    fn test_parse_player_leave_log() {
        let line1 = "[12:35:00 INFO]: Gautam left the game";
        assert_eq!(parse_player_leave_log(line1), Some("Gautam".to_string()));

        let line2 = "[12:35:00 INFO]: Gautam lost connection: Disconnected";
        assert_eq!(parse_player_leave_log(line2), Some("Gautam".to_string()));

        let line3 = "[12:35:00 INFO]: Disconnecting Gautam: Kicked by operator";
        assert_eq!(parse_player_leave_log(line3), Some("Gautam".to_string()));
    }

    #[test]
    fn test_parse_player_op_log() {
        let line1 = "[12:35:00 INFO]: Made Gautam a server operator";
        assert_eq!(parse_player_op_log(line1), Some(("Gautam".to_string(), true)));

        let line2 = "[12:35:00 INFO]: Made Gautam no longer a server operator";
        assert_eq!(parse_player_op_log(line2), Some(("Gautam".to_string(), false)));
    }
}

