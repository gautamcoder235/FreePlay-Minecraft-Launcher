use async_trait::async_trait;
use freeplay_domain::{
    AccountId, AccountIdentity, DomainError, Instance, InstanceId, ServerId, ServerProfile,
    TunnelId, TunnelRun,
};

#[async_trait]
pub trait StoragePort: Send + Sync {
    // Instances
    async fn get_instance(&self, id: &InstanceId) -> Result<Option<Instance>, DomainError>;
    async fn list_instances(&self) -> Result<Vec<Instance>, DomainError>;
    async fn save_instance(&self, instance: &Instance) -> Result<(), DomainError>;
    async fn delete_instance(&self, id: &InstanceId) -> Result<(), DomainError>;

    // Accounts
    async fn get_account(&self, id: &AccountId) -> Result<Option<AccountIdentity>, DomainError>;
    async fn get_active_account(&self) -> Result<Option<AccountIdentity>, DomainError>;
    async fn list_accounts(&self) -> Result<Vec<AccountIdentity>, DomainError>;
    async fn save_account(&self, account: &AccountIdentity) -> Result<(), DomainError>;
    async fn set_active_account(&self, id: &AccountId) -> Result<(), DomainError>;
    async fn delete_account(&self, id: &AccountId) -> Result<(), DomainError>;

    // Servers
    async fn get_server(&self, id: &ServerId) -> Result<Option<ServerProfile>, DomainError>;
    async fn list_servers(&self) -> Result<Vec<ServerProfile>, DomainError>;
    async fn save_server(&self, server: &ServerProfile) -> Result<(), DomainError>;
    async fn delete_server(&self, id: &ServerId) -> Result<(), DomainError>;

    // Tunnels
    async fn get_tunnel(&self, id: &TunnelId) -> Result<Option<TunnelRun>, DomainError>;
    async fn save_tunnel(&self, tunnel: &TunnelRun) -> Result<(), DomainError>;
}
