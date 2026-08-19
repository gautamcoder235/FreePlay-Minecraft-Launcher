use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JavaRuntimeError {
	#[error("Network error: {0}")]
	Network(#[from] reqwest::Error),
	#[error("IO error: {0}")]
	Io(#[from] io::Error),
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("Runtime component '{0}' not found for platform '{1}'")]
	ComponentNotFound(String, String),
}

/// Java version recommendation for a given Minecraft version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JavaMajorVersion {
	Java8 = 8,
	Java17 = 17,
	Java21 = 21,
}

impl JavaMajorVersion {
	/// Determine required Java major version from Minecraft version string (e.g. "1.16.5", "1.20.1", "1.21.4")
	pub fn from_minecraft_version(mc_version: &str) -> Self {
		let parts: Vec<u32> = mc_version
			.split('.')
			.filter_map(|s| s.parse::<u32>().ok())
			.collect();

		if parts.len() >= 2 {
			let major = parts[0];
			let minor = parts[1];
			let patch = parts.get(2).copied().unwrap_or(0);

			if major == 1 {
				if minor <= 16 {
					return Self::Java8;
				} else if minor < 20 || (minor == 20 && patch < 5) {
					return Self::Java17;
				} else {
					return Self::Java21;
				}
			}
		}

		Self::Java21
	}

	pub fn mojang_component_name(&self) -> &'static str {
		match self {
			Self::Java8 => "java-runtime-alpha",
			Self::Java17 => "java-runtime-gamma",
			Self::Java21 => "java-runtime-delta",
		}
	}
}

/// Mojang Java Runtime Manifest Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaManifest {
	#[serde(default)]
	pub linux: HashMap<String, Vec<JavaComponent>>,
	#[serde(default)]
	pub macos: HashMap<String, Vec<JavaComponent>>,
	#[serde(default)]
	pub windows: HashMap<String, Vec<JavaComponent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaComponent {
	pub manifest: JavaFileManifestRef,
	pub version: JavaComponentVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaFileManifestRef {
	pub sha1: String,
	pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaComponentVersion {
	pub name: String,
	pub released: String,
}

pub struct JavaRuntimeManager {
	client: reqwest::Client,
}

impl Default for JavaRuntimeManager {
	fn default() -> Self {
		Self::new()
	}
}

impl JavaRuntimeManager {
	pub fn new() -> Self {
		let client = reqwest::Client::builder()
			.user_agent("FreePlay-Launcher/1.0.0")
			.build()
			.unwrap_or_default();
		Self { client }
	}

	/// Get target OS string used in Mojang runtime manifest ("windows-x64", "linux-x64", "mac-os", etc.)
	pub fn current_mojang_platform() -> &'static str {
		#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
		return "windows-x64";
		#[cfg(all(target_os = "windows", target_arch = "x86"))]
		return "windows-x86";
		#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
		return "windows-arm64";
		#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
		return "linux-x64";
		#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
		return "mac-os-arm64";
		#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
		return "mac-os";
		#[allow(unreachable_code)]
		"windows-x64"
	}

	/// Fetch all Java runtime manifests from Mojang API
	pub async fn fetch_all_runtimes(&self) -> Result<JavaManifest, JavaRuntimeError> {
		let url = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0c9254c0ce99e6a78846fd3e9b18daf708869/all.json";
		let response = self.client.get(url).send().await?;
		let manifest: JavaManifest = response.json().await?;
		Ok(manifest)
	}

	/// Find specific component download link for current OS
	pub fn resolve_component_url<'a>(
		&self,
		manifest: &'a JavaManifest,
		platform: &str,
		component_name: &str,
	) -> Option<&'a JavaFileManifestRef> {
		let platform_map = if platform.starts_with("windows") {
			&manifest.windows
		} else if platform.starts_with("mac") {
			&manifest.macos
		} else {
			&manifest.linux
		};

		platform_map.get(platform).and_then(|components| {
			components
				.iter()
				.find(|c| c.version.name == component_name || component_name.starts_with(&c.version.name))
				.map(|c| &c.manifest)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_java_version_resolution() {
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.12.2"), JavaMajorVersion::Java8);
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.16.5"), JavaMajorVersion::Java8);
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.17.1"), JavaMajorVersion::Java17);
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.20.1"), JavaMajorVersion::Java17);
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.20.6"), JavaMajorVersion::Java21);
		assert_eq!(JavaMajorVersion::from_minecraft_version("1.21.4"), JavaMajorVersion::Java21);
	}
}
