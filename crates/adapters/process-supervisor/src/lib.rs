use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{mpsc, Mutex, RwLock};

use freeplay_domain::DomainError;

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
