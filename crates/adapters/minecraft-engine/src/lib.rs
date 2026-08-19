use async_trait::async_trait;
use reqwest::Client;

use freeplay_application::{JavaRuntimeInfo, MinecraftMetadataPort, VersionManifestEntry};
use freeplay_domain::{DomainError, Instance, InstanceRunState};

pub mod java_detector;
pub mod manifest;
pub mod rules;

pub struct MinecraftEngine {
    http: Client,
}

impl MinecraftEngine {
    pub fn new() -> Self {
        Self {
            http: Client::builder()
                .user_agent("FreePlay-Minecraft-Launcher/0.1.0")
                .build()
                .unwrap_or_default(),
        }
    }
}

impl Default for MinecraftEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MinecraftMetadataPort for MinecraftEngine {
    async fn fetch_version_manifest(&self) -> Result<Vec<VersionManifestEntry>, DomainError> {
        let resp = self
            .http
            .get(manifest::VERSION_MANIFEST_URL)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to fetch Mojang version manifest: {e}")))?;

        let manifest: manifest::RawVersionManifest = resp
            .json()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to parse Mojang manifest JSON: {e}")))?;

        let entries = manifest
            .versions
            .into_iter()
            .map(|v| VersionManifestEntry {
                id: v.id,
                release_type: v.release_type,
                url: v.url,
                release_time: v.release_time,
            })
            .collect();

        Ok(entries)
    }

    async fn detect_installed_java(&self) -> Result<Vec<JavaRuntimeInfo>, DomainError> {
        Ok(java_detector::detect_java_installations())
    }

    async fn prepare_instance_artifacts(&self, _instance: &Instance) -> Result<(), DomainError> {
        // Implementation for downloading client.jar, libraries, assets for instance
        Ok(())
    }

    async fn launch_instance(&self, instance: &mut Instance) -> Result<(), DomainError> {
        instance.transition_to(InstanceRunState::Launching)?;
        Ok(())
    }

    async fn get_instance_run_state(&self, instance: &Instance) -> Result<InstanceRunState, DomainError> {
        Ok(instance.state.clone())
    }

    async fn kill_instance(&self, instance: &mut Instance) -> Result<(), DomainError> {
        instance.transition_to(InstanceRunState::Stopped)?;
        Ok(())
    }
}
