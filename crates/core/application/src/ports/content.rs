use async_trait::async_trait;
use freeplay_domain::{ContentType, DomainError, InstanceId, ModpackIndex};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SearchHit {
    pub project_id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: ContentType,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub author: String,
    pub categories: Vec<String>,
    pub latest_version: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SearchQuery {
    pub query: String,
    pub project_type: Option<ContentType>,
    pub game_version: Option<String>,
    pub loader: Option<String>,
    pub limit: u32,
    pub offset: u32,
}

#[async_trait]
pub trait ContentProviderPort: Send + Sync {
    async fn search_content(&self, query: SearchQuery) -> Result<Vec<SearchHit>, DomainError>;
    async fn parse_mrpack_index(&self, archive_path: &str) -> Result<ModpackIndex, DomainError>;
    async fn install_mod_to_instance(&self, instance_id: &InstanceId, project_id: &str, version_id: Option<&str>) -> Result<(), DomainError>;
    async fn import_mrpack_to_instance(&self, archive_path: &str, instance_name: &str) -> Result<InstanceId, DomainError>;
}
