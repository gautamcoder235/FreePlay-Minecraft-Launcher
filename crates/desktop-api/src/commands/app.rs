use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AppStatusResponse {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub os: String,
    pub architecture: String,
}

#[specta::specta]
pub fn get_app_status() -> AppStatusResponse {
    AppStatusResponse {
        name: "FreePlay Minecraft Launcher".to_string(),
        version: "0.1.0".to_string(),
        environment: "development".to_string(),
        os: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
    }
}
