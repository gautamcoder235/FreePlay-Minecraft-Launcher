pub mod launch_builder;

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
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

pub struct ServerProcessSupervisor {
    child: Arc<Mutex<Option<Child>>>,
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    status: Arc<RwLock<DedicatedServerStatus>>,
    logs: Arc<RwLock<VecDeque<String>>>,
    log_broadcaster: broadcast::Sender<String>,
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

    pub async fn get_status(&self) -> DedicatedServerStatus {
        self.status.read().await.clone()
    }

    pub async fn get_logs(&self) -> Vec<String> {
        self.logs.read().await.iter().cloned().collect()
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

        // 8. Prepare and spawn process
        let mut cmd = Command::new(&java_binary);

        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW (0x08000000) for headless execution
            cmd.creation_flags(0x08000000);
        }

        let ram = if ram_mb < 512 { 2048 } else { ram_mb };
        cmd.arg(format!("-Xms{}M", ram));
        cmd.arg(format!("-Xmx{}M", ram));
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

        // Background log consumer & state updater
        tokio::spawn(async move {
            let mut tasks = Vec::new();
            if let Some(stdout) = stdout {
                let status_inner = status_clone.clone();
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();

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
        });

        *child_lock = Some(child);
        Ok(())
    }

    /// Stops the dedicated server gracefully with fallback to termination
    pub async fn stop_server(&self) -> Result<(), DomainError> {
        *self.status.write().await = DedicatedServerStatus::Stopping;

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

pub struct PlayitTunnelSupervisor {
    child: Arc<Mutex<Option<Child>>>,
    status: Arc<RwLock<PlayitAgentStatus>>,
    claim_url: Arc<RwLock<Option<String>>>,
    public_address: Arc<RwLock<Option<String>>>,
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
            status: Arc::new(RwLock::new(PlayitAgentStatus::Stopped)),
            claim_url: Arc::new(RwLock::new(None)),
            public_address: Arc::new(RwLock::new(None)),
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
        self.status.read().await.clone()
    }

    pub async fn get_claim_url(&self) -> Option<String> {
        self.claim_url.read().await.clone()
    }

    pub async fn get_public_address(&self) -> Option<String> {
        self.public_address.read().await.clone()
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
        if proc_lock.is_some() {
            return Err(DomainError::Internal(
                "Playit tunnel agent is already running".to_string(),
            ));
        }

        *self.claim_url.write().await = None;
        *self.public_address.write().await = None;

        // 1. Locate or download playit binary
        let binary_path = self.resolve_or_download_binary().await?;

        // 2. Update status to Starting
        *self.status.write().await = PlayitAgentStatus::Starting;
        let init_log = "[playit.gg] Initializing freeplay tunnel...".to_string();
        tracing::info!("{init_log}");
        {
            let mut lg = self.logs.write().await;
            if lg.len() >= 1000 {
                lg.pop_front();
            }
            lg.push_back(init_log.clone());
        }
        let _ = self.log_broadcaster.send(init_log);

        // 3. Spawn process
        let mut cmd = Command::new(&binary_path);
        cmd.arg("--claim");

        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW (0x08000000) for headless execution
            cmd.creation_flags(0x08000000);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            DomainError::Internal(format!(
                "Failed to spawn playit agent binary '{:?}': {e}",
                binary_path
            ))
        })?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let status_clone = self.status.clone();
        let claim_clone = self.claim_url.clone();
        let pub_addr_clone = self.public_address.clone();
        let logs_clone = self.logs.clone();
        let broadcaster = self.log_broadcaster.clone();

        tokio::spawn(async move {
            let mut tasks = Vec::new();
            if let Some(stdout) = stdout {
                let status_inner = status_clone.clone();
                let claim_inner = claim_clone.clone();
                let pub_addr_inner = pub_addr_clone.clone();
                let logs_inner = logs_clone.clone();
                let broadcaster_inner = broadcaster.clone();

                tasks.push(tokio::spawn(async move {
                    let mut reader = BufReader::new(stdout).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
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

            *status_clone.write().await = PlayitAgentStatus::Stopped;
        });

        *proc_lock = Some(child);
        Ok(())
    }

    /// Stops the playit tunnel process
    pub async fn stop_playit_tunnel(&self) -> Result<(), DomainError> {
        let mut proc_lock = self.child.lock().await;
        if let Some(mut child) = proc_lock.take() {
            let _ = child.kill().await;
        }
        *self.status.write().await = PlayitAgentStatus::Stopped;
        *self.claim_url.write().await = None;
        *self.public_address.write().await = None;
        Ok(())
    }

    async fn resolve_or_download_binary(&self) -> Result<PathBuf, DomainError> {
        // 1. Check custom path if configured
        if let Some(ref path) = *self.binary_path.read().await {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // 2. Check standard repository asset location `bin/playit/playit.exe` or `bin/playit/playit`
        let exe_name = if cfg!(windows) { "playit.exe" } else { "playit" };
        let repo_bin = PathBuf::from("bin").join("playit").join(exe_name);
        if repo_bin.exists() {
            return Ok(repo_bin);
        }

        // Also check relative to executable directory
        if let Ok(exe_dir) = std::env::current_exe() {
            if let Some(parent) = exe_dir.parent() {
                let candidate = parent.join(exe_name);
                if candidate.exists() {
                    return Ok(candidate);
                }
                let candidate2 = parent.join("bin").join("playit").join(exe_name);
                if candidate2.exists() {
                    return Ok(candidate2);
                }
            }
        }

        // 3. Check system PATH
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let p = dir.join(exe_name);
                if p.exists() {
                    return Ok(p);
                }
            }
        }

        // 4. Download playit binary to tools cache
        *self.status.write().await = PlayitAgentStatus::Downloading;
        let tools_dir = dirs::data_local_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("freeplay")
            .join("tools");
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

/// Extracts a playit.gg claim URL from log lines
pub fn extract_playit_claim_url(line: &str) -> Option<String> {
    if let Some(idx) = line.find("https://playit.gg/claim/") {
        let after = &line[idx..];
        let end = after
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ')' || c == ']')
            .unwrap_or(after.len());
        return Some(after[..end].to_string());
    }
    None
}

/// Extracts a playit public domain/address from log lines (e.g. *.gl.joinmc.link, *.playit.gg)
pub fn extract_playit_public_address(line: &str) -> Option<String> {
    for token in line.split_whitespace() {
        let clean = token.trim_matches(|c: char| {
            c == '"' || c == '\'' || c == '(' || c == ')' || c == '[' || c == ']' || c == ','
        });
        if clean.contains(".joinmc.link") || clean.contains(".playit.gg") {
            let addr = clean.strip_prefix("tcp://").unwrap_or(clean);
            return Some(addr.to_string());
        }
    }
    None
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
            // PaperMC API
            let version_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", version);
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
                .and_then(|arr| arr.last())
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    DomainError::Internal(format!("No PaperMC builds found for version {}", version))
                })?;

            format!(
                "https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar",
                version, latest_build, version, latest_build
            )
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

        let line3 = "Random log output from server";
        assert_eq!(extract_playit_public_address(line3), None);
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
}

