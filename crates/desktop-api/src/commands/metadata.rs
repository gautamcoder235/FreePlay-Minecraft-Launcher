use freeplay_application::{JavaRuntimeInfo, VersionManifestEntry};
use freeplay_domain::DomainError;
use crate::context::AppContext;

#[specta::specta]
pub async fn fetch_version_manifest(
    context: &AppContext,
) -> Result<Vec<VersionManifestEntry>, DomainError> {
    context.metadata_service.fetch_version_manifest().await
}

#[specta::specta]
pub async fn detect_installed_java(
    context: &AppContext,
) -> Result<Vec<JavaRuntimeInfo>, DomainError> {
    context.metadata_service.detect_installed_java().await
}
