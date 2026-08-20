use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlayitStatus {
    Stopped,
    Starting,
    Claiming { claim_url: String },
    Connected { public_address: String },
    Error { message: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TunnelConfig {
    pub local_port: u16,
    pub protocol: String, // "tcp" or "udp"
    pub secret_key: Option<String>,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            local_port: 25565,
            protocol: "tcp".to_string(),
            secret_key: None,
        }
    }
}

pub struct PlayitAgent {
    binary_path: PathBuf,
    status: Arc<RwLock<PlayitStatus>>,
    process: Arc<RwLock<Option<Child>>>,
}

impl PlayitAgent {
    pub fn new(binary_path: PathBuf) -> Self {
        Self {
            binary_path,
            status: Arc::new(RwLock::new(PlayitStatus::Stopped)),
            process: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_status(&self) -> PlayitStatus {
        self.status.read().await.clone()
    }

    /// Starts the playit tunnel process
    pub async fn start(&self, config: TunnelConfig) -> Result<(), String> {
        let mut proc_lock = self.process.write().await;
        if proc_lock.is_some() {
            return Err("Playit agent is already running".to_string());
        }

        if !self.binary_path.exists() {
            return Err(format!(
                "Playit binary not found at {:?}",
                self.binary_path
            ));
        }

        *self.status.write().await = PlayitStatus::Starting;

        let mut cmd = Command::new(&self.binary_path);
        if let Some(ref secret) = config.secret_key {
            cmd.arg("--secret").arg(secret);
        }
        if let Some(parent) = self.binary_path.parent() {
            cmd.current_dir(parent);
        }

        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW (0x08000000) ensures zero console window pops up (100% headless)
            cmd.creation_flags(0x08000000);
            cmd.arg("--socket-path").arg(r"\\.\pipe\freeplay_playit_ipc");
        }

        #[cfg(not(windows))]
        {
            let temp_socket = std::env::temp_dir().join("freeplay_playit_ipc.sock");
            cmd.arg("--socket-path").arg(temp_socket);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn playit agent: {e}"))?;
        let stdout = child.stdout.take();
        *proc_lock = Some(child);

        // Spawn async reader for stdout to capture claim URL or public address
        if let Some(stdout) = stdout {
            let status_clone = self.status.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let line_trimmed = line.trim();
                    if line_trimmed.contains("https://playit.gg/claim/") {
                        if let Some(url_start) = line_trimmed.find("https://playit.gg/claim/") {
                            let url = line_trimmed[url_start..].to_string();
                            *status_clone.write().await = PlayitStatus::Claiming { claim_url: url };
                        }
                    } else if line_trimmed.contains(".joinmc.link") || line_trimmed.contains(".playit.gg") {
                        *status_clone.write().await = PlayitStatus::Connected {
                            public_address: line_trimmed.to_string(),
                        };
                    }
                }
            });
        }

        Ok(())
    }

    /// Stops the playit tunnel process
    pub async fn stop(&self) -> Result<(), String> {
        let mut proc_lock = self.process.write().await;
        if let Some(mut child) = proc_lock.take() {
            let _ = child.kill().await;
        }
        *self.status.write().await = PlayitStatus::Stopped;
        Ok(())
    }
}
