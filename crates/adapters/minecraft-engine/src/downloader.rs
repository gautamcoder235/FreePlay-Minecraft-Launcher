use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
	#[error("Network error: {0}")]
	Network(#[from] reqwest::Error),
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
	#[error("SHA-1 checksum mismatch for {path}: expected {expected}, got {actual}")]
	ChecksumMismatch {
		path: String,
		expected: String,
		actual: String,
	},
	#[error("Zip extraction error: {0}")]
	Zip(#[from] zip::result::ZipError),
}

/// Download item descriptor
#[derive(Debug, Clone)]
pub struct DownloadTask {
	pub url: String,
	pub target_path: PathBuf,
	pub expected_sha1: Option<String>,
}

pub struct EngineDownloader {
	client: reqwest::Client,
}

impl Default for EngineDownloader {
	fn default() -> Self {
		Self::new()
	}
}

impl EngineDownloader {
	pub fn new() -> Self {
		let client = reqwest::Client::builder()
			.user_agent("FreePlay-Launcher/1.0.0")
			.build()
			.unwrap_or_default();
		Self { client }
	}

	/// Download a single file and verify SHA-1 hash if provided
	pub async fn download_file(&self, task: &DownloadTask) -> Result<(), DownloadError> {
		if task.target_path.exists() {
			if let Some(ref expected_sha1) = task.expected_sha1 {
				if Self::verify_sha1(&task.target_path, expected_sha1).is_ok() {
					return Ok(());
				}
			}
		}

		if let Some(parent) = task.target_path.parent() {
			fs::create_dir_all(parent)?;
		}

		let response = self.client.get(&task.url).send().await?;
		let bytes = response.bytes().await?;

		if let Some(ref expected_sha1) = task.expected_sha1 {
			let mut hasher = Sha1::new();
			hasher.update(&bytes);
			let actual_sha1 = hex::encode(hasher.finalize());
			if !actual_sha1.eq_ignore_ascii_case(expected_sha1) {
				return Err(DownloadError::ChecksumMismatch {
					path: task.target_path.display().to_string(),
					expected: expected_sha1.clone(),
					actual: actual_sha1,
				});
			}
		}

		let mut file = File::create(&task.target_path)?;
		file.write_all(&bytes)?;
		Ok(())
	}

	/// Download multiple files in parallel with high throughput
	pub async fn download_batch(&self, tasks: Vec<DownloadTask>, concurrency: usize) -> Vec<Result<(), DownloadError>> {
		use tokio::sync::Semaphore;
		use std::sync::Arc;

		let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
		let mut handles = Vec::new();

		for task in tasks {
			let sem = semaphore.clone();
			let client = self.client.clone();
			handles.push(tokio::spawn(async move {
				let _permit = sem.acquire().await.unwrap();
				let task_item = task;
				let downloader = EngineDownloader { client };
				downloader.download_file(&task_item).await
			}));
		}

		let mut results = Vec::new();
		for handle in handles {
			match handle.await {
				Ok(res) => results.push(res),
				Err(e) => results.push(Err(DownloadError::Io(io::Error::new(io::ErrorKind::Other, e.to_string())))),
			}
		}
		results
	}

	/// Helper to calculate and verify file SHA-1 checksum
	pub fn verify_sha1(path: &Path, expected_sha1: &str) -> Result<(), DownloadError> {
		let mut file = File::open(path)?;
		let mut hasher = Sha1::new();
		let mut buffer = [0u8; 8192];

		loop {
			let count = file.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			hasher.update(&buffer[..count]);
		}

		let actual_sha1 = hex::encode(hasher.finalize());
		if actual_sha1.eq_ignore_ascii_case(expected_sha1) {
			Ok(())
		} else {
			Err(DownloadError::ChecksumMismatch {
				path: path.display().to_string(),
				expected: expected_sha1.to_string(),
				actual: actual_sha1,
			})
		}
	}

	/// Extract native LWJGL libraries (.dll, .so, .dylib) from a JAR into natives_dir
	pub fn extract_natives(jar_path: &Path, natives_dir: &Path) -> Result<(), DownloadError> {
		if !natives_dir.exists() {
			fs::create_dir_all(natives_dir)?;
		}

		let file = File::open(jar_path)?;
		let mut archive = zip::ZipArchive::new(file)?;

		for i in 0..archive.len() {
			let mut entry = archive.by_index(i)?;
			let name = entry.name().to_string();

			// Skip META-INF and non-library files
			if name.starts_with("META-INF/") || entry.is_dir() {
				continue;
			}

			let path = Path::new(&name);
			if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
				let ext_lower = ext.to_lowercase();
				if ext_lower == "dll" || ext_lower == "so" || ext_lower == "dylib" {
					if let Some(filename) = path.file_name() {
						let out_path = natives_dir.join(filename);
						let mut out_file = File::create(out_path)?;
						io::copy(&mut entry, &mut out_file)?;
					}
				}
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[test]
	fn test_verify_sha1() {
		let dir = tempdir().unwrap();
		let file_path = dir.path().join("test.txt");
		fs::write(&file_path, b"FreePlay Launcher Test Engine").unwrap();

		let mut hasher = Sha1::new();
		hasher.update(b"FreePlay Launcher Test Engine");
		let expected_sha1 = hex::encode(hasher.finalize());

		assert!(EngineDownloader::verify_sha1(&file_path, &expected_sha1).is_ok());
		assert!(EngineDownloader::verify_sha1(&file_path, "0000000000000000000000000000000000000000").is_err());
	}
}
