use freeplay_desktop_api::commands::*;
use freeplay_desktop_api::context::AppContext;
use freeplay_domain::LoaderType;

#[tokio::test]
async fn test_desktop_api_flow() {
    let ctx = AppContext::in_memory().unwrap();

    // 1. App status
    let status = get_app_status();
    assert_eq!(status.name, "FreePlay Minecraft Launcher");
    assert_eq!(status.version, "0.1.0");

    // 2. Create offline account
    let account = create_offline_account(&ctx, "AlexPro".to_string())
        .await
        .unwrap();
    assert_eq!(account.minecraft_username, "AlexPro");
    assert_eq!(account.kind, freeplay_domain::AccountKind::Offline);

    let accounts = list_accounts(&ctx).await.unwrap();
    assert_eq!(accounts.len(), 1);

    let active = get_active_account(&ctx).await.unwrap();
    assert!(active.is_some());
    assert_eq!(active.unwrap().minecraft_username, "AlexPro");

    // 3. Create instance
    let instance = create_instance(
        &ctx,
        "Vanilla 1.21.4".to_string(),
        "1.21.4".to_string(),
        LoaderType::Vanilla,
    )
    .await
    .unwrap();
    assert_eq!(instance.name, "Vanilla 1.21.4");

    let instances = list_instances(&ctx).await.unwrap();
    assert_eq!(instances.len(), 1);

    // 4. Java runtime detection
    let java_runtimes = detect_installed_java(&ctx).await.unwrap();
    for j in java_runtimes {
        assert!(j.major_version > 0);
    }
}
