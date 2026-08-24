use earx_lib::state::{AncLevel, AncMode, DeviceManager};
use earx_lib::transport::MockBluetoothTransport;

#[tokio::test]
async fn test_manager_connect_and_init() {
    let mock = MockBluetoothTransport::new();
    let manager = DeviceManager::new(Box::new(mock));

    assert!(!manager.get_state().await.is_connected);

    manager
        .connect("AA:BB:CC:DD:EE:FF", Some("Ayush's CMF Buds 2 Plus".into()))
        .await
        .expect("Connect and init must succeed");

    let state = manager.get_state().await;
    assert!(state.is_connected);
    assert_eq!(state.device_name, "Ayush's CMF Buds 2 Plus");
    assert_eq!(state.battery.left, Some(95));
    assert_eq!(state.battery.right, Some(90));
    assert_eq!(state.battery.case, Some(40));
    assert!(state.battery.is_charging_case);
    assert_eq!(state.anc_mode, AncMode::NoiseCancellation(AncLevel::High));
    assert!(state.eq.ultra_bass_enabled);
    assert_eq!(state.eq.ultra_bass_level, 2);
    assert_eq!(state.firmware_version.as_deref(), Some("1.0.1.37"));
    assert!(state.model.is_some());
    assert_eq!(state.model.unwrap().name, "CMF Buds Pro 2");
}

#[tokio::test]
async fn test_manager_set_anc_and_bass() {
    let mock = MockBluetoothTransport::new();
    let manager = DeviceManager::new(Box::new(mock));

    manager.connect("AA:BB:CC:DD:EE:FF", None).await.unwrap();

    // Set ANC to Transparency
    manager.set_anc_mode(AncMode::Transparency).await.unwrap();
    let state = manager.get_state().await;
    assert_eq!(state.anc_mode, AncMode::Transparency);

    // Set Ultra Bass to Level 4
    manager.set_ultra_bass(true, 4).await.unwrap();
    let state = manager.get_state().await;
    assert_eq!(state.eq.ultra_bass_level, 4);
    assert!(state.eq.ultra_bass_enabled);
}
