use async_trait::async_trait;
use freeplay_domain::{AccountIdentity, DomainError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DeviceCodeChallenge {
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in_seconds: u32,
    pub interval_seconds: u32,
}

#[async_trait]
pub trait AuthPort: Send + Sync {
    async fn start_device_flow(&self) -> Result<DeviceCodeChallenge, DomainError>;
    async fn poll_device_flow(&self, user_code: &str) -> Result<Option<AccountIdentity>, DomainError>;
    async fn refresh_session(&self, account: &AccountIdentity) -> Result<AccountIdentity, DomainError>;
}
