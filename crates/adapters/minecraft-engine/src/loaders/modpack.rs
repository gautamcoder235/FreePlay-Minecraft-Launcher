use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModpackError {
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("Zip extraction error: {0}")]
	Zip(#[from] zip::result::ZipError),
	#[error("Invalid modpack index: missing modrinth.index.json")]
	MissingIndex,
}

/// Modrinth Modpack index schema (modrinth.index.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrpackIndex {
	pub format_version: u32,
	pub game: String,
	pub version_id: String,
	pub name: String,
	#[serde(default)]
	pub summary: Option<String>,
	pub dependencies: HashMap<String, String>,
	pub files: Vec<MrpackFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrpackFile {
	pub path: String,
	pub hashes: HashMap<String, String>,
	#[serde(default)]
	pub env: Option<MrpackFileEnv>,
	pub downloads: Vec<String>,
	pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrpackFileEnv {
	pub client: String,
	pub server: String,
}

pub struct MrpackParser;

impl MrpackParser {
	/// Parse modrinth.index.json from a .mrpack zip archive file
	pub fn parse_mrpack_archive(mrpack_path: &Path) -> Result<MrpackIndex, ModpackError> {
		let file = File::open(mrpack_path)?;
		let mut archive = zip::ZipArchive::new(file)?;

		for i in 0..archive.len() {
			let mut entry = archive.by_index(i)?;
			if entry.name() == "modrinth.index.json" {
				let mut content = String::new();
				entry.read_to_string(&mut content)?;
				let index: MrpackIndex = serde_json::from_str(&content)?;
				return Ok(index);
			}
		}

		Err(ModpackError::MissingIndex)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_mrpack_json_structure() {
		let json = r#"{
			"formatVersion": 1,
			"game": "minecraft",
			"versionId": "1.20.1",
			"name": "FreePlay Performance Pack",
			"dependencies": {
				"minecraft": "1.20.1",
				"fabric-loader": "0.15.0"
			},
			"files": [
				{
					"path": "mods/sodium-fabric-0.5.8+mc1.20.1.jar",
					"hashes": {
						"sha1": "37f48e3592c30080345ff5b5b9e07f9c2d1b8214"
					},
					"downloads": [
						"https://cdn.modrinth.com/data/AANobbF1/versions/sodium.jar"
					],
					"fileSize": 1234567
				}
			]
		}"#;

		let index: MrpackIndex = serde_json::from_str(json).unwrap();
		assert_eq!(index.name, "FreePlay Performance Pack");
		assert_eq!(index.dependencies.get("minecraft").unwrap(), "1.20.1");
		assert_eq!(index.files.len(), 1);
		assert_eq!(index.files[0].path, "mods/sodium-fabric-0.5.8+mc1.20.1.jar");
	}
}
