use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct InstanceId(pub Uuid);

impl InstanceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for InstanceId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for InstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum LoaderType {
    Vanilla,
    Fabric { version: String },
    Forge { version: String },
    NeoForge { version: String },
    Quilt { version: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum InstanceRunState {
    Stopped,
    Preparing,
    Launching,
    Running { pid: u32, started_at: DateTime<Utc> },
    Stopping,
    Crashed { exit_code: Option<i32> },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstalledFile {
    pub relative_path: String,
    pub sha1: String,
    pub sha512: Option<String>,
    pub size_bytes: u64,
    pub source: Option<String>, // e.g. "modrinth:AANobbMI"
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstanceManifest {
    pub format_version: u32,
    pub files: Vec<InstalledFile>,
}

impl Default for InstanceManifest {
    fn default() -> Self {
        Self {
            format_version: 1,
            files: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct InstanceSettings {
    pub min_memory_mb: u32,
    pub max_memory_mb: u32,
    pub java_path: Option<String>,
    pub jvm_args: Vec<String>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for InstanceSettings {
    fn default() -> Self {
        Self {
            min_memory_mb: 2048,
            max_memory_mb: 4096,
            java_path: None,
            jvm_args: vec![
                "-XX:+UseG1GC".to_string(),
                "-XX:+UnlockExperimentalVMOptions".to_string(),
            ],
            window_width: 854,
            window_height: 480,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Instance {
    pub id: InstanceId,
    pub name: String,
    pub icon_path: Option<String>,
    pub game_version: String,
    pub loader: LoaderType,
    pub settings: InstanceSettings,
    pub state: InstanceRunState,
    pub total_play_time_seconds: u64,
    pub last_played_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Instance {
    pub fn new(name: String, game_version: String, loader: LoaderType) -> Result<Self, DomainError> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err(DomainError::Validation("Instance name cannot be empty".to_string()));
        }
        if trimmed_name.len() > 64 {
            return Err(DomainError::Validation("Instance name cannot exceed 64 characters".to_string()));
        }

        let now = Utc::now();
        Ok(Self {
            id: InstanceId::new(),
            name: trimmed_name.to_string(),
            icon_path: None,
            game_version,
            loader,
            settings: InstanceSettings::default(),
            state: InstanceRunState::Stopped,
            total_play_time_seconds: 0,
            last_played_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn transition_to(&mut self, next: InstanceRunState) -> Result<(), DomainError> {
        let valid = match (&self.state, &next) {
            (InstanceRunState::Stopped, InstanceRunState::Preparing) => true,
            (InstanceRunState::Preparing, InstanceRunState::Launching) => true,
            (InstanceRunState::Preparing, InstanceRunState::Stopped) => true, // Cancelled / Failed
            (InstanceRunState::Launching, InstanceRunState::Running { .. }) => true,
            (InstanceRunState::Launching, InstanceRunState::Stopped) => true,
            (InstanceRunState::Running { .. }, InstanceRunState::Stopping) => true,
            (InstanceRunState::Running { .. }, InstanceRunState::Stopped) => true,
            (InstanceRunState::Running { .. }, InstanceRunState::Crashed { .. }) => true,
            (InstanceRunState::Stopping, InstanceRunState::Stopped) => true,
            (InstanceRunState::Crashed { .. }, InstanceRunState::Preparing) => true,
            (InstanceRunState::Crashed { .. }, InstanceRunState::Stopped) => true,
            _ => false,
        };

        if !valid {
            return Err(DomainError::InvalidStateTransition {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
                reason: "Illegal instance state progression".to_string(),
            });
        }

        self.state = next;
        self.updated_at = Utc::now();
        Ok(())
    }
}
