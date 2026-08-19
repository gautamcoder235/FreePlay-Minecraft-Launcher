use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FabricError {
	#[error("Network error: {0}")]
	Network(#[from] reqwest::Error),
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("No loader found for Minecraft {0}")]
	NoLoaderFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricLoaderVersion {
	pub loader: FabricLoaderInfo,
	#[serde(default)]
	pub launcher_meta: Option<FabricLauncherMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricLoaderInfo {
	pub version: String,
	#[serde(default)]
	pub stable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricLauncherMeta {
	pub main_class: Option<FabricMainClass>,
	pub libraries: Option<FabricLibraries>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FabricMainClass {
	Single(String),
	Map { client: String, server: String },
}

impl FabricMainClass {
	pub fn client_main_class(&self) -> &str {
		match self {
			Self::Single(s) => s,
			Self::Map { client, .. } => client,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricLibraries {
	#[serde(default)]
	pub common: Vec<FabricLibrary>,
	#[serde(default)]
	pub client: Vec<FabricLibrary>,
	#[serde(default)]
	pub server: Vec<FabricLibrary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricLibrary {
	pub name: String,
	#[serde(default)]
	pub url: Option<String>,
}

pub struct FabricInstaller {
	client: reqwest::Client,
}

impl Default for FabricInstaller {
	fn default() -> Self {
		Self::new()
	}
}

impl FabricInstaller {
	pub fn new() -> Self {
		let client = reqwest::Client::builder()
			.user_agent("FreePlay-Launcher/1.0.0")
			.build()
			.unwrap_or_default();
		Self { client }
	}

	/// Resolve Fabric loader profile for a game version (e.g. "1.20.1", "1.21.4")
	pub async fn resolve_loader(&self, game_version: &str) -> Result<FabricLoaderVersion, FabricError> {
		let url = format!("https://meta.fabricmc.net/v2/versions/loader/{}", game_version);
		let response = self.client.get(&url).send().await?;
		let versions: Vec<FabricLoaderVersion> = response.json().await?;

		versions
			.into_iter()
			.next()
			.ok_or_else(|| FabricError::NoLoaderFound(game_version.to_string()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_fabric_meta_resolution() {
		let installer = FabricInstaller::new();
		let res = installer.resolve_loader("1.20.1").await;
		let profile = res.unwrap();
		let meta = profile.launcher_meta.unwrap();
		let main_class = meta.main_class.unwrap();
		assert_eq!(main_class.client_main_class(), "net.fabricmc.loader.impl.launch.knot.KnotClient");
	}
}
