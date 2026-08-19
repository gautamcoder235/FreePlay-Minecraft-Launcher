use freeplay_application::MinecraftMetadataPort;
use freeplay_minecraft_engine::rules::is_rule_allowed;
use freeplay_minecraft_engine::manifest::{OsRule, Rule};
use freeplay_minecraft_engine::MinecraftEngine;

#[test]
fn test_rule_evaluator_os_matching() {
    let windows_rule = vec![Rule {
        action: "allow".to_string(),
        os: Some(OsRule {
            name: Some("windows".to_string()),
            arch: None,
        }),
    }];

    assert!(is_rule_allowed(Some(&windows_rule), "windows", "x86_64"));
    assert!(!is_rule_allowed(Some(&windows_rule), "linux", "x86_64"));
    assert!(!is_rule_allowed(Some(&windows_rule), "macos", "aarch64"));
}

#[tokio::test]
async fn test_java_detection_finds_runtime() {
    let engine = MinecraftEngine::new();
    let runtimes = engine.detect_installed_java().await.unwrap();
    
    // We expect at least one valid Java runtime if installed, and every found runtime has major_version > 0
    for runtime in runtimes {
        assert!(runtime.major_version > 0);
        assert!(!runtime.path.is_empty());
    }
}
