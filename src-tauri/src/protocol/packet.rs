use serde::{Deserialize, Serialize};
use thiserror::Error;
use super::crc::compute_crc16;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("Frame too short: length {0} < 10")]
    FrameTooShort(usize),
    #[error("Invalid magic header: expected [0x55, 0x60, 0x01], got {0:?}")]
    InvalidMagic([u8; 3]),
    #[error("Payload length mismatch: header says {expected}, got {actual}")]
    LengthMismatch { expected: usize, actual: usize },
    #[error("CRC mismatch: calculated {calculated:#06x}, received {received:#06x}")]
    CrcMismatch { calculated: u16, received: u16 },
}

/// Commands supported by the Nothing/CMF Bluetooth RFCOMM protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    RequestSerialNumber,  // 0xC006 (49158) -> Resp 0x4006 (16390)
    ReadBattery,          // 0xC007 (49159) -> Resp 0xE001 (57345) / 0x4007 (16391)
    ReadAnc,              // 0xC01E (49182) -> Resp 0xE003 (57347) / 0x401E (16414)
    SetAnc,               // 0xF00F (61455)
    ReadEq,               // 0xC01F (49183) -> Resp 0x401F (16415) / 0x4050 (16464)
    SetEq,                // 0xF010 (61456)
    ReadListeningMode,    // 0xC050 (49232) -> Resp 0x4050 (16464)
    SetListeningMode,     // 0xF01D (61469)
    ReadEnhancedBass,     // 0xC04E (49230) -> Resp 0x404E (16462)
    SetEnhancedBass,      // 0xF051 (61521)
    ReadAdvancedEq,       // 0xC04C (49228) -> Resp 0x404C (16460)
    SetAdvancedEq,        // 0xF04F (61519)
    ReadCustomEq,         // 0xC044 (49220) -> Resp 0x4044 (16452)
    SetCustomEq,          // 0xF041 (61505)
    ReadInEar,            // 0xC00E (49166) -> Resp 0x400E (16398)
    SetInEar,             // 0xF004 (61444)
    ReadLatencyMode,      // 0xC041 (49217) -> Resp 0x4041 (16449)
    SetLatencyMode,       // 0xF040 (61504)
    ReadPersonalizedAnc,  // 0xC020 (49184) -> Resp 0x4020 (16416)
    SetPersonalizedAnc,   // 0xF011 (61457)
    ReadGestures,         // 0xC018 (49176) -> Resp 0x4018 (16408)
    SetGestures,          // 0xF003 (61443)
    ReadFirmware,         // 0xC042 (49218) -> Resp 0x4042 (16450)
    RingBuds,             // 0xF002 (61442)
    FitTest,              // 0xF014 (61460) -> Resp 0xE00D (57357)
    ReadLedCaseColor,     // 0xC017 (49175) -> Resp 0x4017 (16407)
    SetLedCaseColor,      // 0xF00D (61453)
    Unknown(u16),
}

impl Command {
    pub const CODE_REQUEST_SERIAL: u16 = 0xC006;
    pub const CODE_READ_BATTERY: u16 = 0xC007;
    pub const CODE_READ_ANC: u16 = 0xC01E;
    pub const CODE_SET_ANC: u16 = 0xF00F;
    pub const CODE_READ_EQ: u16 = 0xC01F;
    pub const CODE_SET_EQ: u16 = 0xF010;
    pub const CODE_READ_LISTENING_MODE: u16 = 0xC050;
    pub const CODE_SET_LISTENING_MODE: u16 = 0xF01D;
    pub const CODE_READ_ENHANCED_BASS: u16 = 0xC04E;
    pub const CODE_SET_ENHANCED_BASS: u16 = 0xF051;
    pub const CODE_READ_ADVANCED_EQ: u16 = 0xC04C;
    pub const CODE_SET_ADVANCED_EQ: u16 = 0xF04F;
    pub const CODE_READ_CUSTOM_EQ: u16 = 0xC044;
    pub const CODE_SET_CUSTOM_EQ: u16 = 0xF041;
    pub const CODE_READ_IN_EAR: u16 = 0xC00E;
    pub const CODE_SET_IN_EAR: u16 = 0xF004;
    pub const CODE_READ_LATENCY_MODE: u16 = 0xC041;
    pub const CODE_SET_LATENCY_MODE: u16 = 0xF040;
    pub const CODE_READ_PERSONALIZED_ANC: u16 = 0xC020;
    pub const CODE_SET_PERSONALIZED_ANC: u16 = 0xF011;
    pub const CODE_READ_GESTURES: u16 = 0xC018;
    pub const CODE_SET_GESTURES: u16 = 0xF003;
    pub const CODE_READ_FIRMWARE: u16 = 0xC042;
    pub const CODE_RING_BUDS: u16 = 0xF002;
    pub const CODE_FIT_TEST: u16 = 0xF014;
    pub const CODE_READ_LED_CASE_COLOR: u16 = 0xC017;
    pub const CODE_SET_LED_CASE_COLOR: u16 = 0xF00D;

    pub fn code(&self) -> u16 {
        match *self {
            Command::RequestSerialNumber => Self::CODE_REQUEST_SERIAL,
            Command::ReadBattery => Self::CODE_READ_BATTERY,
            Command::ReadAnc => Self::CODE_READ_ANC,
            Command::SetAnc => Self::CODE_SET_ANC,
            Command::ReadEq => Self::CODE_READ_EQ,
            Command::SetEq => Self::CODE_SET_EQ,
            Command::ReadListeningMode => Self::CODE_READ_LISTENING_MODE,
            Command::SetListeningMode => Self::CODE_SET_LISTENING_MODE,
            Command::ReadEnhancedBass => Self::CODE_READ_ENHANCED_BASS,
            Command::SetEnhancedBass => Self::CODE_SET_ENHANCED_BASS,
            Command::ReadAdvancedEq => Self::CODE_READ_ADVANCED_EQ,
            Command::SetAdvancedEq => Self::CODE_SET_ADVANCED_EQ,
            Command::ReadCustomEq => Self::CODE_READ_CUSTOM_EQ,
            Command::SetCustomEq => Self::CODE_SET_CUSTOM_EQ,
            Command::ReadInEar => Self::CODE_READ_IN_EAR,
            Command::SetInEar => Self::CODE_SET_IN_EAR,
            Command::ReadLatencyMode => Self::CODE_READ_LATENCY_MODE,
            Command::SetLatencyMode => Self::CODE_SET_LATENCY_MODE,
            Command::ReadPersonalizedAnc => Self::CODE_READ_PERSONALIZED_ANC,
            Command::SetPersonalizedAnc => Self::CODE_SET_PERSONALIZED_ANC,
            Command::ReadGestures => Self::CODE_READ_GESTURES,
            Command::SetGestures => Self::CODE_SET_GESTURES,
            Command::ReadFirmware => Self::CODE_READ_FIRMWARE,
            Command::RingBuds => Self::CODE_RING_BUDS,
            Command::FitTest => Self::CODE_FIT_TEST,
            Command::ReadLedCaseColor => Self::CODE_READ_LED_CASE_COLOR,
            Command::SetLedCaseColor => Self::CODE_SET_LED_CASE_COLOR,
            Command::Unknown(code) => code,
        }
    }
}

impl From<u16> for Command {
    fn from(val: u16) -> Self {
        match val {
            Self::CODE_REQUEST_SERIAL => Command::RequestSerialNumber,
            Self::CODE_READ_BATTERY => Command::ReadBattery,
            Self::CODE_READ_ANC => Command::ReadAnc,
            Self::CODE_SET_ANC => Command::SetAnc,
            Self::CODE_READ_EQ => Command::ReadEq,
            Self::CODE_SET_EQ => Command::SetEq,
            Self::CODE_READ_LISTENING_MODE => Command::ReadListeningMode,
            Self::CODE_SET_LISTENING_MODE => Command::SetListeningMode,
            Self::CODE_READ_ENHANCED_BASS => Command::ReadEnhancedBass,
            Self::CODE_SET_ENHANCED_BASS => Command::SetEnhancedBass,
            Self::CODE_READ_ADVANCED_EQ => Command::ReadAdvancedEq,
            Self::CODE_SET_ADVANCED_EQ => Command::SetAdvancedEq,
            Self::CODE_READ_CUSTOM_EQ => Command::ReadCustomEq,
            Self::CODE_SET_CUSTOM_EQ => Command::SetCustomEq,
            Self::CODE_READ_IN_EAR => Command::ReadInEar,
            Self::CODE_SET_IN_EAR => Command::SetInEar,
            Self::CODE_READ_LATENCY_MODE => Command::ReadLatencyMode,
            Self::CODE_SET_LATENCY_MODE => Command::SetLatencyMode,
            Self::CODE_READ_PERSONALIZED_ANC => Command::ReadPersonalizedAnc,
            Self::CODE_SET_PERSONALIZED_ANC => Command::SetPersonalizedAnc,
            Self::CODE_READ_GESTURES => Command::ReadGestures,
            Self::CODE_SET_GESTURES => Command::SetGestures,
            Self::CODE_READ_FIRMWARE => Command::ReadFirmware,
            Self::CODE_RING_BUDS => Command::RingBuds,
            Self::CODE_FIT_TEST => Command::FitTest,
            Self::CODE_READ_LED_CASE_COLOR => Command::ReadLedCaseColor,
            Self::CODE_SET_LED_CASE_COLOR => Command::SetLedCaseColor,
            other => Command::Unknown(other),
        }
    }
}

impl From<Command> for u16 {
    fn from(cmd: Command) -> Self {
        cmd.code()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFrame {
    pub command: u16,
    pub sequence: u8,
    pub payload: Vec<u8>,
}

/// Encodes an outbound packet frame with header, payload, and CRC-16.
pub fn encode_frame(command: u16, sequence: u8, payload: &[u8]) -> Vec<u8> {
    let payload_len = payload.len() as u8;
    let cmd_bytes = command.to_le_bytes();

    let mut frame = Vec::with_capacity(8 + payload.len() + 2);
    frame.extend_from_slice(&[0x55, 0x60, 0x01]);
    frame.push(cmd_bytes[0]);
    frame.push(cmd_bytes[1]);
    frame.push(payload_len);
    frame.push(0x00); // reserved
    frame.push(sequence);
    frame.extend_from_slice(payload);

    let crc = compute_crc16(&frame);
    let crc_bytes = crc.to_le_bytes();
    frame.push(crc_bytes[0]);
    frame.push(crc_bytes[1]);

    frame
}

/// Decodes an inbound packet frame, validating header magic, payload length, and CRC-16.
pub fn decode_frame(raw: &[u8]) -> Result<ParsedFrame, ProtocolError> {
    if raw.len() < 10 {
        return Err(ProtocolError::FrameTooShort(raw.len()));
    }

    if raw[0] != 0x55 || raw[1] != 0x60 || raw[2] != 0x01 {
        return Err(ProtocolError::InvalidMagic([raw[0], raw[1], raw[2]]));
    }

    let cmd = u16::from_le_bytes([raw[3], raw[4]]);
    let payload_len = raw[5] as usize;
    let sequence = raw[7];

    let expected_total_len = 8 + payload_len + 2;
    if raw.len() < expected_total_len {
        return Err(ProtocolError::LengthMismatch {
            expected: expected_total_len,
            actual: raw.len(),
        });
    }

    let payload = raw[8..8 + payload_len].to_vec();
    let received_crc = u16::from_le_bytes([raw[8 + payload_len], raw[8 + payload_len + 1]]);
    let calculated_crc = compute_crc16(&raw[0..8 + payload_len]);

    if received_crc != calculated_crc {
        return Err(ProtocolError::CrcMismatch {
            calculated: calculated_crc,
            received: received_crc,
        });
    }

    Ok(ParsedFrame {
        command: cmd,
        sequence,
        payload,
    })
}
