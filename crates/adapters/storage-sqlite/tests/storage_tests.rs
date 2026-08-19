use freeplay_application::StoragePort;
use freeplay_domain::{
    AccountIdentity, Instance, InstanceRunState, LoaderType, ServerEngine, ServerProfile,
    ServerRunState,
};
use freeplay_storage_sqlite::SqliteStorage;

#[tokio::test]
async fn test_instance_crud() {
    let storage = SqliteStorage::open_in_memory().unwrap();

    let mut instance = Instance::new(
        "Survival 1.21.4".to_string(),
        "1.21.4".to_string(),
        LoaderType::Vanilla,
    )
    .unwrap();

    // Save instance
    storage.save_instance(&instance).await.unwrap();

    // Get instance
    let fetched = storage.get_instance(&instance.id).await.unwrap();
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.name, "Survival 1.21.4");
    assert_eq!(fetched.game_version, "1.21.4");
    assert_eq!(fetched.loader, LoaderType::Vanilla);

    // List instances
    let list = storage.list_instances().await.unwrap();
    assert_eq!(list.len(), 1);

    // Update instance state
    instance.transition_to(InstanceRunState::Preparing).unwrap();
    storage.save_instance(&instance).await.unwrap();
    let updated = storage.get_instance(&instance.id).await.unwrap().unwrap();
    assert_eq!(updated.state, InstanceRunState::Preparing);

    // Delete instance
    storage.delete_instance(&instance.id).await.unwrap();
    assert!(storage.get_instance(&instance.id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_offline_and_microsoft_accounts_storage() {
    let storage = SqliteStorage::open_in_memory().unwrap();

    // Create offline account
    let offline_acc = AccountIdentity::new_offline("ShadowCrafter".to_string()).unwrap();
    storage.save_account(&offline_acc).await.unwrap();

    // Create microsoft account
    let ms_uuid = uuid::Uuid::new_v4();
    let ms_acc = AccountIdentity::new_microsoft("StevePro".to_string(), ms_uuid, None, true).unwrap();
    storage.save_account(&ms_acc).await.unwrap();

    // Set offline as active
    storage.set_active_account(&offline_acc.id).await.unwrap();

    let active = storage.get_active_account().await.unwrap().unwrap();
    assert_eq!(active.minecraft_username, "ShadowCrafter");
    assert_eq!(active.kind, freeplay_domain::AccountKind::Offline);

    let all = storage.list_accounts().await.unwrap();
    assert_eq!(all.len(), 2);
}

#[tokio::test]
async fn test_server_storage() {
    let storage = SqliteStorage::open_in_memory().unwrap();

    let mut server = ServerProfile::new(
        "Local Paper Server".to_string(),
        "1.21.4".to_string(),
        ServerEngine::Paper { build: Some(120) },
    )
    .unwrap();

    server.agree_to_eula();
    server.transition_to(ServerRunState::Preparing).unwrap();

    storage.save_server(&server).await.unwrap();

    let fetched = storage.get_server(&server.id).await.unwrap().unwrap();
    assert_eq!(fetched.name, "Local Paper Server");
    assert!(fetched.eula.agreed);
    assert_eq!(fetched.state, ServerRunState::Preparing);
}
