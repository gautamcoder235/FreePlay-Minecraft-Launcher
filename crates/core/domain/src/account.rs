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
pub enum AccountKind {
    /// Official Microsoft account (MSA) with Xbox Live authentication
    Microsoft,
    /// Offline / Custom username profile (like SKlauncher / TLauncher)
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementStatus {
    Entitled,
    NotEntitled,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AccountIdentity {
    pub id: AccountId,
    pub kind: AccountKind,
    pub minecraft_username: String,
    pub minecraft_uuid: Uuid,
    pub skin_url: Option<String>,
    pub entitlement: EntitlementStatus,
    pub is_active: bool,
    pub added_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Generates a standard Minecraft offline UUID from a username.
/// Uses the exact Java algorithm: UUID.nameUUIDFromBytes(("OfflinePlayer:" + username).getBytes(UTF_8))
pub fn generate_offline_uuid(username: &str) -> Uuid {
    // Java UUID.nameUUIDFromBytes uses MD5 hash with version 3 and RFC 4122 variant
    let input = format!("OfflinePlayer:{}", username);
    let digest = md5::compute(input.as_bytes());
    let mut bytes = digest.0;
    
    // Set version to 3 (0x30)
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    // Set variant to IETF RFC 4122 (0x80)
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    
    Uuid::from_bytes(bytes)
}

impl AccountIdentity {
    /// Creates a new official Microsoft account profile
    pub fn new_microsoft(
        username: String,
        minecraft_uuid: Uuid,
        skin_url: Option<String>,
        entitled: bool,
    ) -> Result<Self, DomainError> {
        let trimmed_username = username.trim();
        if trimmed_username.is_empty() {
            return Err(DomainError::Validation("Username cannot be empty".to_string()));
        }

        Ok(Self {
            id: AccountId::new(),
            kind: AccountKind::Microsoft,
            minecraft_username: trimmed_username.to_string(),
            minecraft_uuid,
            skin_url,
            entitlement: if entitled {
                EntitlementStatus::Entitled
            } else {
                EntitlementStatus::NotEntitled
            },
            is_active: false,
            added_at: Utc::now(),
            last_used_at: None,
        })
    }

    /// Creates a new Offline / Custom player profile (instant setup, no purchase required)
    pub fn new_offline(username: String) -> Result<Self, DomainError> {
        let trimmed = username.trim();
        if trimmed.is_empty() {
            return Err(DomainError::Validation("Player name cannot be empty".to_string()));
        }
        if trimmed.len() > 16 {
            return Err(DomainError::Validation("Minecraft usernames cannot exceed 16 characters".to_string()));
        }
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(DomainError::Validation("Minecraft usernames can only contain letters, numbers, and underscores".to_string()));
        }

        let offline_uuid = generate_offline_uuid(trimmed);

        Ok(Self {
            id: AccountId::new(),
            kind: AccountKind::Offline,
            minecraft_username: trimmed.to_string(),
            minecraft_uuid: offline_uuid,
            skin_url: None,
            entitlement: EntitlementStatus::Offline,
            is_active: false,
            added_at: Utc::now(),
            last_used_at: None,
        })
    }
}
