use std::sync::Arc;
use freeplay_domain::{AccountId, AccountIdentity, DomainError};
use crate::ports::{KeyringPort, StoragePort};

pub struct AccountService {
    storage: Arc<dyn StoragePort>,
    keyring: Arc<dyn KeyringPort>,
}

impl AccountService {
    pub fn new(storage: Arc<dyn StoragePort>, keyring: Arc<dyn KeyringPort>) -> Self {
        Self { storage, keyring }
    }

    /// Creates an offline account profile (instant setup, no password or internet required)
    pub async fn create_offline_account(&self, username: String) -> Result<AccountIdentity, DomainError> {
        let account = AccountIdentity::new_offline(username)?;
        self.storage.save_account(&account).await?;

        // If there is no active account, make this one active by default
        if self.storage.get_active_account().await?.is_none() {
            self.storage.set_active_account(&account.id).await?;
        }

        Ok(account)
    }

    pub async fn list_accounts(&self) -> Result<Vec<AccountIdentity>, DomainError> {
        self.storage.list_accounts().await
    }

    pub async fn get_active_account(&self) -> Result<Option<AccountIdentity>, DomainError> {
        self.storage.get_active_account().await
    }

    pub async fn set_active_account(&self, id: &AccountId) -> Result<(), DomainError> {
        self.storage.set_active_account(id).await
    }

    pub async fn delete_account(&self, id: &AccountId) -> Result<(), DomainError> {
        self.keyring.delete_token(id).await?;
        self.storage.delete_account(id).await
    }
}
