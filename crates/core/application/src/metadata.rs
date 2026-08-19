use std::sync::Arc;
use freeplay_domain::DomainError;
use crate::ports::{JavaRuntimeInfo, MinecraftMetadataPort, VersionManifestEntry};

pub struct MetadataService {
    minecraft: Arc<dyn MinecraftMetadataPort>,
}

impl MetadataService {
    pub fn new(minecraft: Arc<dyn MinecraftMetadataPort>) -> Self {
        Self { minecraft }
    }

    pub async fn fetch_version_manifest(&self) -> Result<Vec<VersionManifestEntry>, DomainError> {
        self.minecraft.fetch_version_manifest().await
    }

    pub async fn detect_installed_java(&self) -> Result<Vec<JavaRuntimeInfo>, DomainError> {
        self.minecraft.detect_installed_java().await
    }
}
