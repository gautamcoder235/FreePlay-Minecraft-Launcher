use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfflineSkinError {
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
	#[error("Invalid image format or size")]
	InvalidImage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkinVariant {
	Classic,
	Slim,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineSkinMetadata {
	pub username: String,
	pub texture_hash: String,
	pub variant: SkinVariant,
	pub cape_url: Option<String>,
}

pub struct OfflineSkinStore {
	skins_dir: PathBuf,
}

impl OfflineSkinStore {
	pub fn new(skins_dir: PathBuf) -> Self {
		if !skins_dir.exists() {
			let _ = fs::create_dir_all(&skins_dir);
		}
		Self { skins_dir }
	}

	/// Save a raw skin PNG image for an offline user
	pub fn save_skin(&self, username: &str, png_bytes: &[u8], variant: SkinVariant) -> Result<OfflineSkinMetadata, OfflineSkinError> {
		if png_bytes.is_empty() {
			return Err(OfflineSkinError::InvalidImage);
		}

		let mut hasher = Sha256::new();
		hasher.update(png_bytes);
		let hash = format!("{:x}", hasher.finalize());

		let file_name = format!("{}_{}.png", username.to_lowercase(), hash);
		let target_path = self.skins_dir.join(&file_name);
		let mut file = File::create(&target_path)?;
		file.write_all(png_bytes)?;

		let meta = OfflineSkinMetadata {
			username: username.to_string(),
			texture_hash: hash,
			variant,
			cape_url: None,
		};

		let meta_path = self.skins_dir.join(format!("{}.json", username.to_lowercase()));
		let meta_json = serde_json::to_string(&meta).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
		fs::write(meta_path, meta_json)?;

		Ok(meta)
	}

	/// Load offline skin metadata for a username
	pub fn get_skin(&self, username: &str) -> Option<OfflineSkinMetadata> {
		let meta_path = self.skins_dir.join(format!("{}.json", username.to_lowercase()));
		if meta_path.exists() {
			if let Ok(content) = fs::read_to_string(meta_path) {
				return serde_json::from_str(&content).ok();
			}
		}
		None
	}

	/// Get path to saved skin PNG file
	pub fn get_skin_file_path(&self, username: &str) -> Option<PathBuf> {
		if let Some(meta) = self.get_skin(username) {
			let file_name = format!("{}_{}.png", username.to_lowercase(), meta.texture_hash);
			let path = self.skins_dir.join(file_name);
			if path.exists() {
				return Some(path);
			}
		}
		None
	}

	/// Delete offline skin for a user
	pub fn delete_skin(&self, username: &str) -> Result<(), OfflineSkinError> {
		if let Some(meta) = self.get_skin(username) {
			let file_name = format!("{}_{}.png", username.to_lowercase(), meta.texture_hash);
			let _ = fs::remove_file(self.skins_dir.join(file_name));
		}
		let meta_path = self.skins_dir.join(format!("{}.json", username.to_lowercase()));
		if meta_path.exists() {
			fs::remove_file(meta_path)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[test]
	fn test_offline_skin_store_lifecycle() {
		let dir = tempdir().unwrap();
		let store = OfflineSkinStore::new(dir.path().to_path_buf());

		let dummy_png = b"PNG_HEADER_DUMMY_TEXTURE_DATA";
		let meta = store.save_skin("ShadowCrafter", dummy_png, SkinVariant::Classic).unwrap();

		assert_eq!(meta.username, "ShadowCrafter");
		assert!(store.get_skin("ShadowCrafter").is_some());
		assert!(store.get_skin_file_path("ShadowCrafter").unwrap().exists());

		store.delete_skin("ShadowCrafter").unwrap();
		assert!(store.get_skin("ShadowCrafter").is_none());
	}
}
