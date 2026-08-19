use async_trait::async_trait;
use freeplay_domain::{DomainError, Instance, InstanceRunState};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct JavaRuntimeInfo {
    pub path: String,
    pub major_version: u32,
    pub is_64bit: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct VersionManifestEntry {
    pub id: String,
    pub release_type: String,
    pub url: String,
    pub release_time: String,
}

#[async_trait]
pub trait MinecraftMetadataPort: Send + Sync {
    async fn fetch_version_manifest(&self) -> Result<Vec<VersionManifestEntry>, DomainError>;
    async fn detect_installed_java(&self) -> Result<Vec<JavaRuntimeInfo>, DomainError>;
    async fn prepare_instance_artifacts(&self, instance: &Instance) -> Result<(), DomainError>;
    async fn launch_instance(&self, instance: &mut Instance) -> Result<(), DomainError>;
    async fn get_instance_run_state(&self, instance: &Instance) -> Result<InstanceRunState, DomainError>;
    async fn kill_instance(&self, instance: &mut Instance) -> Result<(), DomainError>;
}
