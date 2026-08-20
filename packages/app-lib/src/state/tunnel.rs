use crate::state::FriendsSocket;
use crate::state::friends::{TunnelSockets, WriteSocket};
use ariadne::networking::message::ClientToServerMessage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use uuid::Uuid;

pub(super) enum InternalTunnelSocket {
    Listening(SocketAddr),
    Connected(Mutex<OwnedWriteHalf>),
}

pub struct TunnelSocket {
    pub(super) socket_id: Uuid,
    pub(super) write: WriteSocket,
    pub(super) sockets: TunnelSockets,
    pub(super) internal: Arc<InternalTunnelSocket>,
}

impl TunnelSocket {
    pub fn socket_id(&self) -> Uuid {
        self.socket_id
    }

    pub async fn shutdown(self) -> crate::Result<()> {
        if self.sockets.remove(&self.socket_id).is_some() {
            FriendsSocket::send_message(
                &self.write,
                ClientToServerMessage::SocketClose {
                    socket: self.socket_id,
                },
            )
            .await?;
            if let InternalTunnelSocket::Connected(ref stream) =
                *self.internal.clone()
            {
                stream.lock().await.shutdown().await?
            }
        }
        Ok(())
    }
}

impl Drop for TunnelSocket {
    fn drop(&mut self) {
        if self.sockets.remove(&self.socket_id).is_some() {
            let write = self.write.clone();
            let socket_id = self.socket_id;
            tokio::spawn(async move {
                let _ = FriendsSocket::send_message(
                    &write,
                    ClientToServerMessage::SocketClose { socket: socket_id },
                )
                .await;
            });
        }
    }
}

// =========================================================================
// Server Hosting & Playit Tunnel Manager State
// =========================================================================

use freeplay_process_supervisor::{
    DedicatedServerStatus, PlayitAgentStatus, PlayitTunnelSupervisor, ServerProcessSupervisor,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostStatus {
    pub server_running: bool,
    pub server_status: DedicatedServerStatus,
    pub server_version: Option<String>,
    pub server_type: Option<String>,
    pub server_ram_mb: u32,
    pub server_port: u16,
    pub tunnel_status: PlayitAgentStatus,
    pub claim_url: Option<String>,
    pub public_address: Option<String>,
    pub tunnels: Vec<freeplay_process_supervisor::PlayitTunnelEntry>,
    pub server_logs: Vec<String>,
    pub tunnel_logs: Vec<String>,
    pub uptime_seconds: u64,
}

pub struct ServerHostingState {
    pub server_supervisor: Arc<ServerProcessSupervisor>,
    pub tunnel_supervisor: Arc<PlayitTunnelSupervisor>,
    pub current_version: Arc<RwLock<Option<String>>>,
    pub current_server_type: Arc<RwLock<Option<String>>>,
    pub current_ram_mb: Arc<RwLock<u32>>,
    pub current_port: Arc<RwLock<u16>>,
    pub current_working_dir: Arc<RwLock<Option<PathBuf>>>,
}

impl Default for ServerHostingState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHostingState {
    pub fn new() -> Self {
        Self {
            server_supervisor: Arc::new(ServerProcessSupervisor::new()),
            tunnel_supervisor: Arc::new(PlayitTunnelSupervisor::new()),
            current_version: Arc::new(RwLock::new(None)),
            current_server_type: Arc::new(RwLock::new(None)),
            current_ram_mb: Arc::new(RwLock::new(2048)),
            current_port: Arc::new(RwLock::new(25565)),
            current_working_dir: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start_server(
        &self,
        version: String,
        server_type: String,
        ram_mb: u32,
        port: u16,
        working_dir: PathBuf,
    ) -> crate::Result<()> {
        *self.current_version.write().await = Some(version.clone());
        *self.current_server_type.write().await = Some(server_type.clone());
        *self.current_ram_mb.write().await = ram_mb;
        *self.current_port.write().await = port;
        *self.current_working_dir.write().await = Some(working_dir.clone());

        self.server_supervisor
            .start_server(&version, &server_type, ram_mb, port, &working_dir)
            .await
            .map_err(crate::Error::from)?;

        Ok(())
    }

    pub async fn stop_server(&self) -> crate::Result<()> {
        let _ = self.tunnel_supervisor.stop_playit_tunnel().await;
        self.server_supervisor.stop_server().await.map_err(crate::Error::from)
    }

    pub async fn send_command(&self, command: String) -> crate::Result<()> {
        self.server_supervisor
            .send_console_command(&command)
            .await
            .map_err(crate::Error::from)
    }

    pub async fn get_status(&self) -> HostStatus {
        let server_running = self.server_supervisor.is_running().await;
        let server_status = self.server_supervisor.get_status().await;
        let server_version = self.current_version.read().await.clone();
        let server_type = self.current_server_type.read().await.clone();
        let server_ram_mb = *self.current_ram_mb.read().await;
        let server_port = *self.current_port.read().await;
        let tunnel_status = self.tunnel_supervisor.get_status().await;
        let claim_url = self.tunnel_supervisor.get_claim_url().await;
        let public_address = self.tunnel_supervisor.get_public_address().await;
        let tunnels = self.tunnel_supervisor.get_tunnels().await;
        let server_logs = self.server_supervisor.get_logs().await;
        let tunnel_logs = self.tunnel_supervisor.get_logs().await;
        let uptime_seconds = self.server_supervisor.get_uptime_seconds().await;

        HostStatus {
            server_running,
            server_status,
            server_version,
            server_type,
            server_ram_mb,
            server_port,
            tunnel_status,
            claim_url,
            public_address,
            tunnels,
            server_logs,
            tunnel_logs,
            uptime_seconds,
        }
    }

    pub async fn start_tunnel(&self, port: u16) -> crate::Result<()> {
        self.tunnel_supervisor
            .start_playit_tunnel(port)
            .await
            .map_err(crate::Error::from)
    }

    pub async fn stop_tunnel(&self) -> crate::Result<()> {
        self.tunnel_supervisor
            .stop_playit_tunnel()
            .await
            .map_err(crate::Error::from)
    }

    pub async fn kill_server(&self) -> crate::Result<()> {
        let _ = self.tunnel_supervisor.stop_playit_tunnel().await;
        self.server_supervisor
            .kill_server()
            .await
            .map_err(crate::Error::from)
    }

    pub async fn get_working_dir(&self) -> Option<std::path::PathBuf> {
        self.current_working_dir.read().await.clone()
    }

    pub async fn set_working_dir(&self, path: std::path::PathBuf) {
        *self.current_working_dir.write().await = Some(path);
    }

    pub async fn update_config(
        &self,
        version: Option<String>,
        engine: Option<String>,
        ram_gb: Option<u32>,
        motd: Option<String>,
        port: Option<u16>,
    ) {
        if let Some(v) = version {
            *self.current_version.write().await = Some(v);
        }
        if let Some(e) = engine {
            *self.current_server_type.write().await = Some(e);
        }
        if let Some(r) = ram_gb {
            *self.current_ram_mb.write().await = r * 1024;
        }
        if let Some(p) = port {
            *self.current_port.write().await = p;
        }
        if let Some(m) = motd {
            if let Some(ref dir) = *self.current_working_dir.read().await {
                let prop_file = dir.join("server.properties");
                if prop_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(&prop_file) {
                        let mut lines: Vec<String> =
                            content.lines().map(|s| s.to_string()).collect();
                        let mut found = false;
                        for line in &mut lines {
                            if line.trim().starts_with("motd=") {
                                *line = format!("motd={}", m);
                                found = true;
                            }
                        }
                        if !found {
                            lines.push(format!("motd={}", m));
                        }
                        let _ = std::fs::write(&prop_file, lines.join("\n") + "\n");
                    }
                }
            }
        }
    }
}

