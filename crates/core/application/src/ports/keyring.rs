use async_trait::async_trait;
use freeplay_domain::{AccountId, DomainError};

#[async_trait]
pub trait KeyringPort: Send + Sync {
    async fn store_token(&self, account_id: &AccountId, token: &str) -> Result<(), DomainError>;
    async fn get_token(&self, account_id: &AccountId) -> Result<Option<String>, DomainError>;
    async fn delete_token(&self, account_id: &AccountId) -> Result<(), DomainError>;

    async fn store_secret(&self, key_name: &str, secret: &str) -> Result<(), DomainError>;
    async fn get_secret(&self, key_name: &str) -> Result<Option<String>, DomainError>;
    async fn delete_secret(&self, key_name: &str) -> Result<(), DomainError>;
}
