use earx_lib::protocol::{decode_frame, encode_frame, Command};
use earx_lib::transport::{BluetoothTransport, MockBluetoothTransport};

#[tokio::test]
async fn test_mock_connect_and_read_serial() {
    let mut transport = MockBluetoothTransport::new();
    assert!(!transport.is_connected().await);

    transport.connect("AA:BB:CC:DD:EE:FF").await.expect("Connect must succeed");
    assert!(transport.is_connected().await);

    // Send request serial number
    let req = encode_frame(Command::RequestSerialNumber.into(), 1, &[]);
    transport.send(&req).await.expect("Send must succeed");

    let resp_bytes = transport.receive().await.expect("Receive must succeed");
    let resp = decode_frame(&resp_bytes).expect("Decode response must succeed");
    assert_eq!(resp.command, 0x4006);
    let text = String::from_utf8_lossy(&resp.payload);
    assert!(text.contains("SH247900123456"));
}

#[tokio::test]
async fn test_mock_battery_and_anc() {
    let mut transport = MockBluetoothTransport::new();
    transport.connect("AA:BB:CC:DD:EE:FF").await.unwrap();

    // Read battery
    let req_bat = encode_frame(Command::ReadBattery.into(), 2, &[]);
    transport.send(&req_bat).await.unwrap();
    let resp_bat = transport.receive().await.unwrap();
    let decoded_bat = decode_frame(&resp_bat).unwrap();
    assert_eq!(decoded_bat.command, 0xE001);
    assert_eq!(decoded_bat.payload[0], 3); // 3 devices
    assert_eq!(decoded_bat.payload[1], 0x02); // Left earbud
    assert_eq!(decoded_bat.payload[2] & 0x7F, 95); // 95%

    // Set ANC to Transparency (1)
    let set_anc = encode_frame(Command::SetAnc.into(), 3, &[0x01, 0x01, 0x00]);
    transport.send(&set_anc).await.unwrap();
    let resp_anc = transport.receive().await.unwrap();
    let decoded_anc = decode_frame(&resp_anc).unwrap();
    assert_eq!(decoded_anc.command, 0xE003);
    assert_eq!(decoded_anc.payload[1], 0x01); // Transparency
}
