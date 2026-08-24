use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::protocol::{decode_frame, encode_frame};
use super::trait_def::{BluetoothTransport, DiscoveredDevice, TransportError};

#[derive(Debug, Clone)]
pub struct MockDeviceState {
    pub serial_number: String,
    pub battery_left: u8,
    pub battery_right: u8,
    pub battery_case: u8,
    pub is_charging_case: bool,
    pub is_charging_left: bool,
    pub is_charging_right: bool,
    pub anc_mode: u8,          // 5 = High, 7 = Mid, 3 = Low, 1 = Trans, 2 = Off, 4 = Adaptive
    pub ultra_bass_enabled: bool,
    pub ultra_bass_level: u8,  // 0 .. 5 (sent as level * 2)
    pub in_ear_detection: bool,
    pub low_latency_mode: bool,
    pub spatial_audio: bool,
    pub firmware_version: String,
}

impl Default for MockDeviceState {
    fn default() -> Self {
        Self {
            serial_number: "SH247900123456".to_string(), // CMF Buds Pro 2 Blue
            battery_left: 95,
            battery_right: 90,
            battery_case: 40,
            is_charging_case: true,
            is_charging_left: false,
            is_charging_right: false,
            anc_mode: 5, // High ANC
            ultra_bass_enabled: true,
            ultra_bass_level: 2,
            in_ear_detection: true,
            low_latency_mode: false,
            spatial_audio: false,
            firmware_version: "1.0.1.37".to_string(),
        }
    }
}

pub struct MockBluetoothTransport {
    connected: bool,
    device_address: String,
    state: Arc<Mutex<MockDeviceState>>,
    inbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

impl MockBluetoothTransport {
    pub fn new() -> Self {
        Self {
            connected: false,
            device_address: String::new(),
            state: Arc::new(Mutex::new(MockDeviceState::default())),
            inbox: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn with_state(state: MockDeviceState) -> Self {
        Self {
            connected: false,
            device_address: String::new(),
            state: Arc::new(Mutex::new(state)),
            inbox: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    async fn handle_command(&self, cmd: u16, seq: u8, payload: &[u8]) -> Vec<u8> {
        let mut state = self.state.lock().await;
        match cmd {
            // 0xC006 (Request Serial Number) -> Resp 0x4006
            0xC006 => {
                let text = format!("\x01\x00\x00\x00\x00\x00\x001,4,{}\n", state.serial_number);
                encode_frame(0x4006, seq, text.as_bytes())
            }
            // 0xC007 (Read Battery) -> Resp 0xE001
            0xC007 => {
                let case_charging_mask = if state.is_charging_case { 0x80 } else { 0x00 };
                let left_charging_mask = if state.is_charging_left { 0x80 } else { 0x00 };
                let right_charging_mask = if state.is_charging_right { 0x80 } else { 0x00 };

                let bat_payload = vec![
                    3, // count
                    0x02, state.battery_left | left_charging_mask,
                    0x04, state.battery_case | case_charging_mask,
                    0x03, state.battery_right | right_charging_mask,
                ];
                encode_frame(0xE001, seq, &bat_payload)
            }
            // 0xC01E (Read ANC) -> Resp 0xE003
            0xC01E => {
                let anc_payload = vec![0x00, state.anc_mode];
                encode_frame(0xE003, seq, &anc_payload)
            }
            // 0xF00F (Set ANC) -> Ack / Resp 0xE003
            0xF00F => {
                if payload.len() >= 2 {
                    state.anc_mode = payload[1];
                }
                let anc_payload = vec![0x00, state.anc_mode];
                encode_frame(0xE003, seq, &anc_payload)
            }
            // 0xC04E (Read Enhanced Bass) -> Resp 0x404E
            0xC04E => {
                let enabled_byte = if state.ultra_bass_enabled { 0x01 } else { 0x00 };
                let level_byte = state.ultra_bass_level * 2;
                encode_frame(0x404E, seq, &[enabled_byte, level_byte])
            }
            // 0xF051 (Set Enhanced Bass) -> Resp 0x404E
            0xF051 => {
                if payload.len() >= 2 {
                    state.ultra_bass_enabled = payload[0] == 0x01;
                    state.ultra_bass_level = payload[1] / 2;
                }
                let enabled_byte = if state.ultra_bass_enabled { 0x01 } else { 0x00 };
                let level_byte = state.ultra_bass_level * 2;
                encode_frame(0x404E, seq, &[enabled_byte, level_byte])
            }
            // 0xC042 (Read Firmware) -> Resp 0x4042
            0xC042 => {
                encode_frame(0x4042, seq, state.firmware_version.as_bytes())
            }
            // 0xC00E (Read In-Ear) -> Resp 0x400E
            0xC00E => {
                let val = if state.in_ear_detection { 0x01 } else { 0x00 };
                encode_frame(0x400E, seq, &[0x00, 0x00, val])
            }
            // 0xF004 (Set In-Ear)
            0xF004 => {
                if payload.len() >= 3 {
                    state.in_ear_detection = payload[2] == 0x01;
                }
                let val = if state.in_ear_detection { 0x01 } else { 0x00 };
                encode_frame(0x400E, seq, &[0x00, 0x00, val])
            }
            // 0xC041 (Read Latency) -> Resp 0x4041
            0xC041 => {
                let val = if state.low_latency_mode { 0x01 } else { 0x02 };
                encode_frame(0x4041, seq, &[val])
            }
            // 0xF040 (Set Latency)
            0xF040 => {
                if !payload.is_empty() {
                    state.low_latency_mode = payload[0] == 0x01;
                }
                let val = if state.low_latency_mode { 0x01 } else { 0x02 };
                encode_frame(0x4041, seq, &[val])
            }
            _ => {
                encode_frame(cmd, seq, &[])
            }
        }
    }
}

#[async_trait]
impl BluetoothTransport for MockBluetoothTransport {
    async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        Ok(vec![
            DiscoveredDevice {
                name: "Ayush's CMF Buds 2 Plus".into(),
                address: "AA:BB:CC:DD:EE:FF".into(),
                is_connected: self.connected,
            },
            DiscoveredDevice {
                name: "Nothing Ear (2)".into(),
                address: "11:22:33:44:55:66".into(),
                is_connected: false,
            },
        ])
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        self.connected = true;
        self.device_address = address.to_string();
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    async fn send(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::Disconnected("Device not connected".into()));
        }

        let parsed = decode_frame(frame).map_err(|e| TransportError::Protocol(format!("{:?}", e)))?;
        let response = self.handle_command(parsed.command, parsed.sequence, &parsed.payload).await;

        let mut inbox = self.inbox.lock().await;
        inbox.push_back(response);
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        if !self.connected {
            return Err(TransportError::Disconnected("Device not connected".into()));
        }

        let mut inbox = self.inbox.lock().await;
        if let Some(packet) = inbox.pop_front() {
            Ok(packet)
        } else {
            Ok(Vec::new())
        }
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.connected = false;
        self.device_address.clear();
        let mut inbox = self.inbox.lock().await;
        inbox.clear();
        Ok(())
    }
}
