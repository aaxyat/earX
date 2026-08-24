use earx_lib::protocol::{
    compute_crc16, decode_frame, encode_frame, format_float_for_eq, from_format_float_for_eq,
    Command, ProtocolError,
};

#[test]
fn test_encode_and_decode_frame() {
    let payload = vec![0x01, 0x05, 0x00]; // ANC Set command
    let encoded = encode_frame(Command::SetAnc.into(), 42, &payload);

    assert_eq!(&encoded[0..3], &[0x55, 0x60, 0x01]);
    assert_eq!(encoded[7], 42); // sequence number
    assert_eq!(encoded.len(), 8 + payload.len() + 2);

    let decoded = decode_frame(&encoded).expect("Decoding must succeed");
    assert_eq!(decoded.command, Command::SetAnc.code());
    assert_eq!(decoded.sequence, 42);
    assert_eq!(decoded.payload, payload);
}

#[test]
fn test_decode_invalid_crc() {
    let payload = vec![0x01, 0x02];
    let mut encoded = encode_frame(Command::ReadBattery.into(), 1, &payload);
    
    // Corrupt payload
    encoded[8] ^= 0xFF;

    let err = decode_frame(&encoded).unwrap_err();
    match err {
        ProtocolError::CrcMismatch { .. } => {}
        _ => panic!("Expected CrcMismatch error, got {:?}", err),
    }
}

#[test]
fn test_decode_invalid_magic() {
    let mut frame = vec![0x00, 0x60, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let crc = compute_crc16(&frame[0..8]);
    let crc_bytes = crc.to_le_bytes();
    frame[8] = crc_bytes[0];
    frame[9] = crc_bytes[1];

    let err = decode_frame(&frame).unwrap_err();
    assert_eq!(err, ProtocolError::InvalidMagic([0x00, 0x60, 0x01]));
}

#[test]
fn test_eq_float_precision_conversions() {
    let gains = [-6.0f32, -3.5, -1.0, 0.0, 0.5, 2.0, 4.5, 6.0];
    for &g in &gains {
        let bytes = format_float_for_eq(g, false);
        let decoded = from_format_float_for_eq(bytes);
        assert!((g - decoded).abs() < 1e-4, "Mismatch on gain {}: got {}", g, decoded);
    }
}
