use freeplay_domain::{AccountId, AccountIdentity, DomainError};
use crate::context::AppContext;

#[specta::specta]
pub async fn list_accounts(context: &AppContext) -> Result<Vec<AccountIdentity>, DomainError> {
    context.account_service.list_accounts().await
}

#[specta::specta]
pub async fn get_active_account(context: &AppContext) -> Result<Option<AccountIdentity>, DomainError> {
    context.account_service.get_active_account().await
}

#[specta::specta]
pub async fn create_offline_account(
    context: &AppContext,
    username: String,
) -> Result<AccountIdentity, DomainError> {
    context.account_service.create_offline_account(username).await
}

#[specta::specta]
pub async fn set_active_account(
    context: &AppContext,
    account_id: AccountId,
) -> Result<(), DomainError> {
    context.account_service.set_active_account(&account_id).await
}

#[specta::specta]
pub async fn delete_account(
    context: &AppContext,
    account_id: AccountId,
) -> Result<(), DomainError> {
    context.account_service.delete_account(&account_id).await
}
