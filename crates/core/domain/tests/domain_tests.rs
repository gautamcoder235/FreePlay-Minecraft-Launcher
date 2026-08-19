use freeplay_domain::{
    DomainError, Instance, InstanceRunState, LoaderType, ServerEngine, ServerProfile,
    ServerRunState, TunnelRun, TunnelStatus,
};

#[test]
fn test_instance_creation_validation() {
    // Empty name should fail
    let err = Instance::new("   ".to_string(), "1.21.4".to_string(), LoaderType::Vanilla);
    assert!(matches!(err, Err(DomainError::Validation(_))));

    // Valid name should succeed
    let instance = Instance::new("My Survival World".to_string(), "1.21.4".to_string(), LoaderType::Vanilla);
    assert!(instance.is_ok());
    let inst = instance.unwrap();
    assert_eq!(inst.name, "My Survival World");
    assert_eq!(inst.state, InstanceRunState::Stopped);
}

#[test]
fn test_instance_state_transitions() {
    let mut instance = Instance::new("Test Instance".to_string(), "1.21.4".to_string(), LoaderType::Vanilla).unwrap();

    // Stopped -> Preparing is valid
    assert!(instance.transition_to(InstanceRunState::Preparing).is_ok());

    // Preparing -> Launching is valid
    assert!(instance.transition_to(InstanceRunState::Launching).is_ok());

    // Launching -> Running is valid
    assert!(instance.transition_to(InstanceRunState::Running {
        pid: 12345,
        started_at: chrono::Utc::now(),
    }).is_ok());

    // Running -> Preparing is INVALID and must error
    let invalid_err = instance.transition_to(InstanceRunState::Preparing);
    assert!(matches!(invalid_err, Err(DomainError::InvalidStateTransition { .. })));
}

#[test]
fn test_server_eula_enforcement() {
    let mut server = ServerProfile::new("Test Paper Server".to_string(), "1.21.4".to_string(), ServerEngine::Vanilla).unwrap();

    // Trying to prepare/start without EULA agreement MUST fail
    let err = server.transition_to(ServerRunState::Preparing);
    assert!(matches!(err, Err(DomainError::Validation(_))));

    // Agree to EULA
    server.agree_to_eula();
    assert!(server.eula.agreed);

    // Now preparing should succeed
    assert!(server.transition_to(ServerRunState::Preparing).is_ok());
}

#[test]
fn test_tunnel_lifecycle() {
    let mut tunnel = TunnelRun::new(freeplay_domain::ProviderKind::Playit);
    assert_eq!(tunnel.status, TunnelStatus::Disabled);

    assert!(tunnel.transition_to(TunnelStatus::Connecting).is_ok());
    assert!(tunnel.transition_to(TunnelStatus::Online {
        public_endpoint: "freeplay-test.joinmc.link:25565".to_string(),
        connected_at: chrono::Utc::now(),
    }).is_ok());
}
