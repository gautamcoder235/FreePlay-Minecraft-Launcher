use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
    Datapack,
    Plugin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct FileHash {
    pub sha1: String,
    pub sha512: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum EnvRequirement {
    Required,
    Optional,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct EnvSupport {
    pub client: EnvRequirement,
    pub server: EnvRequirement,
}

impl Default for EnvSupport {
    fn default() -> Self {
        Self {
            client: EnvRequirement::Required,
            server: EnvRequirement::Optional,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ModpackFile {
    pub path: String,
    pub hashes: HashMap<String, String>,
    pub env: Option<EnvSupport>,
    pub downloads: Vec<String>,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ModpackIndex {
    pub format_version: u32,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<ModpackFile>,
    pub dependencies: HashMap<String, String>,
}
