use std::path::PathBuf;
use std::sync::Arc;
use freeplay_application::{AccountService, InstanceService, MetadataService};
use freeplay_domain::DomainError;
use freeplay_keyring_vault::OsKeyringVault;
use freeplay_minecraft_engine::MinecraftEngine;
use freeplay_storage_sqlite::SqliteStorage;

pub struct AppContext {
    pub instance_service: Arc<InstanceService>,
    pub account_service: Arc<AccountService>,
    pub metadata_service: Arc<MetadataService>,
}

impl AppContext {
    pub fn new_with_sqlite_path(db_path: PathBuf) -> Result<Self, DomainError> {
        let storage = Arc::new(SqliteStorage::open(db_path)?);
        let keyring = Arc::new(OsKeyringVault::default_service());
        let minecraft = Arc::new(MinecraftEngine::new());

        let instance_service = Arc::new(InstanceService::new(storage.clone(), minecraft.clone()));
        let account_service = Arc::new(AccountService::new(storage.clone(), keyring.clone()));
        let metadata_service = Arc::new(MetadataService::new(minecraft.clone()));

        Ok(Self {
            instance_service,
            account_service,
            metadata_service,
        })
    }

    pub fn in_memory() -> Result<Self, DomainError> {
        let storage = Arc::new(SqliteStorage::open_in_memory()?);
        let keyring = Arc::new(OsKeyringVault::default_service());
        let minecraft = Arc::new(MinecraftEngine::new());

        let instance_service = Arc::new(InstanceService::new(storage.clone(), minecraft.clone()));
        let account_service = Arc::new(AccountService::new(storage.clone(), keyring.clone()));
        let metadata_service = Arc::new(MetadataService::new(minecraft.clone()));

        Ok(Self {
            instance_service,
            account_service,
            metadata_service,
        })
    }
}
