use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct TunnelId(pub Uuid);

impl TunnelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TunnelId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TunnelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Manual,
    Playit,
    Fake,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum TunnelStatus {
    Disabled,
    Claiming { claim_url: String },
    Connecting,
    Online {
        public_endpoint: String,
        connected_at: DateTime<Utc>,
    },
    Degraded {
        reason: String,
    },
    Stopping,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TunnelConfig {
    pub provider: ProviderKind,
    pub local_port: u16,
    pub protocol: String, // "tcp" or "udp"
    pub secret_key: Option<String>,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            provider: ProviderKind::Playit,
            local_port: 25565,
            protocol: "tcp".to_string(),
            secret_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TunnelRun {
    pub id: TunnelId,
    pub provider: ProviderKind,
    pub status: TunnelStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TunnelRun {
    pub fn new(provider: ProviderKind) -> Self {
        let now = Utc::now();
        Self {
            id: TunnelId::new(),
            provider,
            status: TunnelStatus::Disabled,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn transition_to(&mut self, next: TunnelStatus) -> Result<(), DomainError> {
        self.status = next;
        self.updated_at = Utc::now();
        Ok(())
    }
}
