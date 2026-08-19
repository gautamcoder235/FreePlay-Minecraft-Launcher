use async_trait::async_trait;
use freeplay_domain::{DomainError, ServerEngine, ServerProfile, ServerRunState};

#[async_trait]
pub trait ServerProviderPort: Send + Sync {
    fn engine(&self) -> ServerEngine;
    async fn install_or_update(&self, profile: &ServerProfile) -> Result<(), DomainError>;
    async fn start_server(&self, profile: &mut ServerProfile) -> Result<(), DomainError>;
    async fn get_server_status(&self, profile: &ServerProfile) -> Result<ServerRunState, DomainError>;
    async fn send_command(&self, profile: &ServerProfile, command: &str) -> Result<(), DomainError>;
    async fn stop_server(&self, profile: &mut ServerProfile) -> Result<(), DomainError>;
}
