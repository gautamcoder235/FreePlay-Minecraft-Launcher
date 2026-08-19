use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct AccountId(pub Uuid);

impl AccountId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AccountId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementStatus {
    Entitled,
    NotEntitled,
    PendingCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AccountIdentity {
    pub id: AccountId,
    pub minecraft_username: String,
    pub minecraft_uuid: Uuid,
    pub skin_url: Option<String>,
    pub entitlement: EntitlementStatus,
    pub is_active: bool,
    pub added_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl AccountIdentity {
    pub fn new(
        username: String,
        minecraft_uuid: Uuid,
        skin_url: Option<String>,
        entitlement: EntitlementStatus,
    ) -> Result<Self, DomainError> {
        let trimmed_username = username.trim();
        if trimmed_username.is_empty() {
            return Err(DomainError::Validation("Username cannot be empty".to_string()));
        }

        Ok(Self {
            id: AccountId::new(),
            minecraft_username: trimmed_username.to_string(),
            minecraft_uuid,
            skin_url,
            entitlement,
            is_active: false,
            added_at: Utc::now(),
            last_used_at: None,
        })
    }
}
