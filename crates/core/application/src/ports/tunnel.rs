use async_trait::async_trait;
use freeplay_domain::{DomainError, ProviderKind, TunnelConfig, TunnelStatus};

#[async_trait]
pub trait TunnelProviderPort: Send + Sync {
    fn provider_kind(&self) -> ProviderKind;
    async fn start_tunnel(&self, config: TunnelConfig) -> Result<(), DomainError>;
    async fn get_status(&self) -> Result<TunnelStatus, DomainError>;
    async fn stop_tunnel(&self) -> Result<(), DomainError>;
}
