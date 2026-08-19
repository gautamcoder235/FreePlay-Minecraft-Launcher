use std::path::PathBuf;

/// Launch Configuration descriptor for a Minecraft client instance
#[derive(Debug, Clone)]
pub struct LaunchConfig {
	pub username: String,
	pub uuid: String,
	pub access_token: String,
	pub user_type: String,
	pub version_id: String,
	pub game_dir: PathBuf,
	pub assets_dir: PathBuf,
	pub asset_index: String,
	pub natives_dir: PathBuf,
	pub main_class: String,
	pub classpath: Vec<PathBuf>,
	pub min_memory_mb: u32,
	pub max_memory_mb: u32,
	pub custom_jvm_args: Vec<String>,
	pub server_ip: Option<String>,
	pub server_port: Option<u16>,
	pub custom_skin_url: Option<String>,
	pub custom_skin_variant: Option<String>,
}

pub struct LaunchArgumentBuilder;

impl LaunchArgumentBuilder {
	/// Construct JVM arguments list
	pub fn build_jvm_args(config: &LaunchConfig) -> Vec<String> {
		let mut args = Vec::new();

		// Memory allocation
		args.push(format!("-Xms{}m", config.min_memory_mb.max(512)));
		args.push(format!("-Xmx{}m", config.max_memory_mb.max(1024)));

		// FreePlay Launcher branding & system properties
		args.push(format!("-Djava.library.path={}", config.natives_dir.display()));
		args.push("-Dminecraft.launcher.brand=FreePlay".to_string());
		args.push("-Dminecraft.launcher.version=1.0.0".to_string());

		// Offline Custom Skin Injection
		if let Some(ref skin_url) = config.custom_skin_url {
			let model = config.custom_skin_variant.as_deref().unwrap_or("default");
			let extra_json = format!(
				r#"{{"textures":{{"SKIN":{{"url":"{}","model":"{}"}}}}}}"#,
				skin_url, model
			);
			args.push(format!("-Dcustomskinloader.extraJson={}", extra_json));
		}

		// Append custom JVM args provided by user
		for custom_arg in &config.custom_jvm_args {
			if !custom_arg.trim().is_empty() {
				args.push(custom_arg.clone());
			}
		}

		// Classpath formulation
		let cp_sep = if cfg!(target_os = "windows") { ";" } else { ":" };
		let cp_joined = config
			.classpath
			.iter()
			.map(|p| p.display().to_string())
			.collect::<Vec<_>>()
			.join(cp_sep);

		args.push("-cp".to_string());
		args.push(cp_joined);

		// Main Class
		args.push(config.main_class.clone());

		args
	}

	/// Construct Game arguments list
	pub fn build_game_args(config: &LaunchConfig) -> Vec<String> {
		let mut args = Vec::new();

		args.push("--username".to_string());
		args.push(config.username.clone());

		args.push("--version".to_string());
		args.push(config.version_id.clone());

		args.push("--gameDir".to_string());
		args.push(config.game_dir.display().to_string());

		args.push("--assetsDir".to_string());
		args.push(config.assets_dir.display().to_string());

		args.push("--assetIndex".to_string());
		args.push(config.asset_index.clone());

		args.push("--uuid".to_string());
		args.push(config.uuid.clone());

		args.push("--accessToken".to_string());
		args.push(config.access_token.clone());

		args.push("--userType".to_string());
		args.push(config.user_type.clone());

		if let Some(ref ip) = config.server_ip {
			args.push("--server".to_string());
			args.push(ip.clone());
		}

		if let Some(port) = config.server_port {
			args.push("--port".to_string());
			args.push(port.to_string());
		}

		args
	}

	/// Construct full execution arguments list (JVM args + Game args)
	pub fn build_full_args(config: &LaunchConfig) -> Vec<String> {
		let mut full = Self::build_jvm_args(config);
		full.extend(Self::build_game_args(config));
		full
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_build_launch_args_offline() {
		let config = LaunchConfig {
			username: "ShadowCrafter".to_string(),
			uuid: "cb2a1b92-4f81-3c81-8b22-54a7bc9d1234".to_string(),
			access_token: "0".to_string(),
			user_type: "legacy".to_string(),
			version_id: "1.20.1".to_string(),
			game_dir: PathBuf::from("/minecraft/instances/default"),
			assets_dir: PathBuf::from("/minecraft/assets"),
			asset_index: "1.20".to_string(),
			natives_dir: PathBuf::from("/minecraft/instances/default/natives"),
			main_class: "net.minecraft.client.main.Main".to_string(),
			classpath: vec![PathBuf::from("/minecraft/libraries/lwjgl.jar")],
			min_memory_mb: 2048,
			max_memory_mb: 4096,
			custom_jvm_args: vec![],
			server_ip: None,
			server_port: None,
			custom_skin_url: None,
			custom_skin_variant: None,
		};

		let jvm_args = LaunchArgumentBuilder::build_jvm_args(&config);
		assert!(jvm_args.contains(&"-Xms2048m".to_string()));
		assert!(jvm_args.contains(&"-Xmx4096m".to_string()));
		assert!(jvm_args.contains(&"-Dminecraft.launcher.brand=FreePlay".to_string()));
		assert!(jvm_args.contains(&"net.minecraft.client.main.Main".to_string()));

		let game_args = LaunchArgumentBuilder::build_game_args(&config);
		assert!(game_args.contains(&"--username".to_string()));
		assert!(game_args.contains(&"ShadowCrafter".to_string()));
		assert!(game_args.contains(&"--accessToken".to_string()));
		assert!(game_args.contains(&"0".to_string()));
		assert!(game_args.contains(&"--userType".to_string()));
		assert!(game_args.contains(&"legacy".to_string()));
	}

	#[test]
	fn test_build_launch_args_custom_skin() {
		let config = LaunchConfig {
			username: "ShadowCrafter".to_string(),
			uuid: "cb2a1b92-4f81-3c81-8b22-54a7bc9d1234".to_string(),
			access_token: "0".to_string(),
			user_type: "legacy".to_string(),
			version_id: "1.20.1".to_string(),
			game_dir: PathBuf::from("/minecraft/instances/default"),
			assets_dir: PathBuf::from("/minecraft/assets"),
			asset_index: "1.20".to_string(),
			natives_dir: PathBuf::from("/minecraft/instances/default/natives"),
			main_class: "net.minecraft.client.main.Main".to_string(),
			classpath: vec![PathBuf::from("/minecraft/libraries/lwjgl.jar")],
			min_memory_mb: 2048,
			max_memory_mb: 4096,
			custom_jvm_args: vec![],
			server_ip: None,
			server_port: None,
			custom_skin_url: Some("http://localhost:1420/skins/shadow.png".to_string()),
			custom_skin_variant: Some("slim".to_string()),
		};

		let jvm_args = LaunchArgumentBuilder::build_jvm_args(&config);
		assert!(jvm_args.iter().any(|arg| arg.contains("-Dcustomskinloader.extraJson=")));
		assert!(jvm_args.iter().any(|arg| arg.contains("http://localhost:1420/skins/shadow.png")));
	}
}
