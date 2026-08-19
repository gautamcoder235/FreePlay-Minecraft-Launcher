use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct ServerId(pub Uuid);

impl ServerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ServerId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ServerEngine {
    Vanilla,
    Paper { build: Option<u32> },
    Purpur { build: Option<u32> },
    Fabric { loader_version: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct EulaAgreement {
    pub agreed: bool,
    pub agreed_at: Option<DateTime<Utc>>,
    pub eula_version: String,
}

impl Default for EulaAgreement {
    fn default() -> Self {
        Self {
            agreed: false,
            agreed_at: None,
            eula_version: "Mojang-2020-05".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ServerRunState {
    Stopped,
    Preparing,
    Starting,
    Ready {
        port: u16,
        online_players: u32,
        max_players: u32,
        started_at: DateTime<Utc>,
    },
    Stopping,
    Crashed {
        exit_code: Option<i32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ServerSettings {
    pub port: u16,
    pub min_memory_mb: u32,
    pub max_memory_mb: u32,
    pub online_mode: bool,
    pub motd: String,
    pub max_players: u32,
    pub view_distance: u32,
    pub pvp: bool,
    pub allow_flight: bool,
    pub difficulty: String,
    pub gamemode: String,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            port: 25565,
            min_memory_mb: 2048,
            max_memory_mb: 4096,
            online_mode: true,
            motd: "A FreePlay Minecraft Server".to_string(),
            max_players: 20,
            view_distance: 10,
            pvp: true,
            allow_flight: false,
            difficulty: "normal".to_string(),
            gamemode: "survival".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ServerProfile {
    pub id: ServerId,
    pub name: String,
    pub game_version: String,
    pub engine: ServerEngine,
    pub eula: EulaAgreement,
    pub settings: ServerSettings,
    pub state: ServerRunState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ServerProfile {
    pub fn new(name: String, game_version: String, engine: ServerEngine) -> Result<Self, DomainError> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err(DomainError::Validation("Server name cannot be empty".to_string()));
        }

        let now = Utc::now();
        Ok(Self {
            id: ServerId::new(),
            name: trimmed_name.to_string(),
            game_version,
            engine,
            eula: EulaAgreement::default(),
            settings: ServerSettings::default(),
            state: ServerRunState::Stopped,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn agree_to_eula(&mut self) {
        self.eula.agreed = true;
        self.eula.agreed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn transition_to(&mut self, next: ServerRunState) -> Result<(), DomainError> {
        if matches!(next, ServerRunState::Starting | ServerRunState::Preparing) && !self.eula.agreed {
            return Err(DomainError::Validation(
                "Cannot start Minecraft server without explicit EULA agreement".to_string(),
            ));
        }

        let valid = match (&self.state, &next) {
            (ServerRunState::Stopped, ServerRunState::Preparing) => true,
            (ServerRunState::Preparing, ServerRunState::Starting) => true,
            (ServerRunState::Preparing, ServerRunState::Stopped) => true,
            (ServerRunState::Starting, ServerRunState::Ready { .. }) => true,
            (ServerRunState::Starting, ServerRunState::Stopped) => true,
            (ServerRunState::Ready { .. }, ServerRunState::Stopping) => true,
            (ServerRunState::Ready { .. }, ServerRunState::Crashed { .. }) => true,
            (ServerRunState::Stopping, ServerRunState::Stopped) => true,
            (ServerRunState::Crashed { .. }, ServerRunState::Preparing) => true,
            (ServerRunState::Crashed { .. }, ServerRunState::Stopped) => true,
            _ => false,
        };

        if !valid {
            return Err(DomainError::InvalidStateTransition {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
                reason: "Illegal server state progression".to_string(),
            });
        }

        self.state = next;
        self.updated_at = Utc::now();
        Ok(())
    }
}
