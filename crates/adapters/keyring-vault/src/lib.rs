use async_trait::async_trait;
use freeplay_application::KeyringPort;
use freeplay_domain::{AccountId, DomainError};
use keyring::Entry;

pub struct OsKeyringVault {
    service_name: String,
}

impl OsKeyringVault {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    pub fn default_service() -> Self {
        Self::new("FreePlay-Minecraft-Launcher")
    }
}

#[async_trait]
impl KeyringPort for OsKeyringVault {
    async fn store_token(&self, account_id: &AccountId, token: &str) -> Result<(), DomainError> {
        let entry = Entry::new(&self.service_name, &format!("account_{}", account_id))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        entry
            .set_password(token)
            .map_err(|e| DomainError::Internal(format!("Failed to store token in OS vault: {e}")))?;
        Ok(())
    }

    async fn get_token(&self, account_id: &AccountId) -> Result<Option<String>, DomainError> {
        let entry = Entry::new(&self.service_name, &format!("account_{}", account_id))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(DomainError::Internal(format!("Failed to retrieve token from OS vault: {e}"))),
        }
    }

    async fn delete_token(&self, account_id: &AccountId) -> Result<(), DomainError> {
        let entry = Entry::new(&self.service_name, &format!("account_{}", account_id))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(DomainError::Internal(format!("Failed to delete token from OS vault: {e}"))),
        }
    }

    async fn store_secret(&self, key_name: &str, secret: &str) -> Result<(), DomainError> {
        let entry = Entry::new(&self.service_name, &format!("secret_{}", key_name))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        entry
            .set_password(secret)
            .map_err(|e| DomainError::Internal(format!("Failed to store secret in OS vault: {e}")))?;
        Ok(())
    }

    async fn get_secret(&self, key_name: &str) -> Result<Option<String>, DomainError> {
        let entry = Entry::new(&self.service_name, &format!("secret_{}", key_name))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        match entry.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(DomainError::Internal(format!("Failed to retrieve secret from OS vault: {e}"))),
        }
    }

    async fn delete_secret(&self, key_name: &str) -> Result<(), DomainError> {
        let entry = Entry::new(&self.service_name, &format!("secret_{}", key_name))
            .map_err(|e| DomainError::Internal(format!("Failed to create keyring entry: {e}")))?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(DomainError::Internal(format!("Failed to delete secret from OS vault: {e}"))),
        }
    }
}
