use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio::time::sleep;
use tracing::{debug, info, warn};

use crate::models::{get_device_model_from_serial, parse_serial_number_payload};
use crate::protocol::{
    decode_frame, encode_frame, format_float_for_eq, Command, ParsedFrame,
};
use crate::transport::{BluetoothTransport, DiscoveredDevice, TransportError};
use super::device_state::{AncLevel, AncMode, BatteryTelemetry, DeviceState};

pub struct DeviceManager {
    transport: Arc<Mutex<Box<dyn BluetoothTransport>>>,
    state: Arc<Mutex<DeviceState>>,
    sequence_id: Arc<Mutex<u8>>,
    event_sender: broadcast::Sender<DeviceState>,
}

impl DeviceManager {
    pub fn new(transport: Box<dyn BluetoothTransport>) -> Self {
        let (tx, _) = broadcast::channel(32);
        Self {
            transport: Arc::new(Mutex::new(transport)),
            state: Arc::new(Mutex::new(DeviceState::default())),
            sequence_id: Arc::new(Mutex::new(1)),
            event_sender: tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DeviceState> {
        self.event_sender.subscribe()
    }

    pub async fn get_state(&self) -> DeviceState {
        self.state.lock().await.clone()
    }

    fn next_sequence(&self, lock: &mut u8) -> u8 {
        let current = *lock;
        if *lock >= 250 {
            *lock = 1;
        } else {
            *lock += 1;
        }
        current
    }

    async fn send_command(&self, cmd: Command, payload: &[u8]) -> Result<(), TransportError> {
        let mut seq_lock = self.sequence_id.lock().await;
        let seq = self.next_sequence(&mut seq_lock);
        let frame = encode_frame(cmd.into(), seq, payload);

        let mut transport = self.transport.lock().await;
        transport.send(&frame).await
    }

    pub async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        let transport = self.transport.lock().await;
        transport.scan_devices().await
    }

    pub async fn connect(&self, address: &str, name: Option<String>) -> Result<(), TransportError> {
        info!("Connecting to Bluetooth device {} ({:?})", address, name);
        {
            let mut transport = self.transport.lock().await;
            transport.connect(address).await?;
        }

        {
            let mut state = self.state.lock().await;
            state.is_connected = true;
            state.address = address.to_string();
            if let Some(n) = name {
                state.device_name = n;
            }
        }

        self.emit_state().await;

        // Run full device initialisation handshake with pacing
        self.init_device().await?;
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), TransportError> {
        info!("Disconnecting active Bluetooth transport");
        {
            let mut transport = self.transport.lock().await;
            transport.disconnect().await?;
        }

        {
            let mut state = self.state.lock().await;
            state.is_connected = false;
        }

        self.emit_state().await;
        Ok(())
    }

    /// Initializes device telemetry and parameters with 100ms inter-command pacing
    pub async fn init_device(&self) -> Result<(), TransportError> {
        info!("Beginning device handshake & initialization sequence");

        // 1. Request Serial Number (for model & capability identification)
        self.send_command(Command::RequestSerialNumber, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 2. Read Battery Levels
        self.send_command(Command::ReadBattery, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 3. Read ANC Status
        self.send_command(Command::ReadAnc, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 4. Read Firmware
        self.send_command(Command::ReadFirmware, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 5. Read In-Ear Detection
        self.send_command(Command::ReadInEar, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 6. Read Low Latency Mode
        self.send_command(Command::ReadLatencyMode, &[]).await?;
        self.read_and_process_responses().await?;
        sleep(Duration::from_millis(100)).await;

        // 7. Read Enhanced Bass
        self.send_command(Command::ReadEnhancedBass, &[]).await?;
        self.read_and_process_responses().await?;

        info!("Device handshake & initialisation complete");
        self.emit_state().await;
        Ok(())
    }

    pub async fn read_and_process_responses(&self) -> Result<(), TransportError> {
        let raw_bytes = {
            let mut transport = self.transport.lock().await;
            transport.receive().await?
        };

        if raw_bytes.is_empty() {
            return Ok(());
        }

        match decode_frame(&raw_bytes) {
            Ok(parsed) => {
                self.process_packet(parsed).await;
            }
            Err(e) => {
                warn!("Received invalid or corrupted packet: {:?}", e);
            }
        }

        Ok(())
    }

    pub async fn process_packet(&self, packet: ParsedFrame) {
        debug!("Processing packet cmd={:#06x}, payload_len={}", packet.command, packet.payload.len());
        let mut state = self.state.lock().await;

        match packet.command {
            // 0x4006: Serial Number Response
            0x4006 => {
                if let Some(serial) = parse_serial_number_payload(&packet.payload) {
                    info!("Received hardware serial: {}", serial);
                    if let Some(model_info) = get_device_model_from_serial(&serial) {
                        info!("Identified model: {}", model_info.name);
                        state.model = Some(model_info);
                    }
                    state.serial_number = Some(serial);
                }
            }
            // 0xE001 / 0x4007: Battery Telemetry Response
            0xE001 | 0x4007 => {
                if !packet.payload.is_empty() {
                    let count = packet.payload[0] as usize;
                    let mut bat = BatteryTelemetry::default();

                    for i in 0..count {
                        if 1 + i * 2 + 1 < packet.payload.len() {
                            let device_id = packet.payload[1 + i * 2];
                            let raw_level = packet.payload[2 + i * 2];
                            let level = raw_level & 0x7F;
                            let is_charging = (raw_level & 0x80) != 0;

                            match device_id {
                                0x02 => {
                                    bat.left = Some(level);
                                    bat.is_charging_left = is_charging;
                                }
                                0x03 => {
                                    bat.right = Some(level);
                                    bat.is_charging_right = is_charging;
                                }
                                0x04 => {
                                    bat.case = Some(level);
                                    bat.is_charging_case = is_charging;
                                }
                                _ => {}
                            }
                        }
                    }
                    state.battery = bat;
                }
            }
            // 0xE003 / 0x401E: ANC Status Response
            0xE003 | 0x401E => {
                if packet.payload.len() >= 2 {
                    let status = packet.payload[1];
                    state.anc_mode = match status {
                        0x05 => AncMode::NoiseCancellation(AncLevel::High),
                        0x07 => AncMode::NoiseCancellation(AncLevel::Mid),
                        0x03 => AncMode::NoiseCancellation(AncLevel::Low),
                        0x04 => AncMode::NoiseCancellation(AncLevel::Adaptive),
                        0x01 => AncMode::Transparency,
                        0x02 => AncMode::Off,
                        _ => AncMode::NoiseCancellation(AncLevel::High),
                    };
                }
            }
            // 0x404E: Enhanced Bass Response
            0x404E => {
                if packet.payload.len() >= 2 {
                    state.eq.ultra_bass_enabled = packet.payload[0] == 0x01;
                    state.eq.ultra_bass_level = packet.payload[1] / 2;
                }
            }
            // 0x4042: Firmware Version Response
            0x4042 => {
                let fw = String::from_utf8_lossy(&packet.payload).to_string();
                state.firmware_version = Some(fw);
            }
            // 0x400E: In-Ear Detection Response
            0x400E => {
                if packet.payload.len() >= 3 {
                    state.in_ear_detection = packet.payload[2] == 0x01;
                }
            }
            // 0x4041: Latency Mode Response
            0x4041 => {
                if !packet.payload.is_empty() {
                    state.low_latency_mode = packet.payload[0] == 0x01;
                }
            }
            _ => {}
        }
    }

    pub async fn set_anc_mode(&self, mode: AncMode) -> Result<(), TransportError> {
        info!("Setting ANC mode to: {:?}", mode);
        let byte_val = match mode {
            AncMode::NoiseCancellation(AncLevel::High) => 0x05,
            AncMode::NoiseCancellation(AncLevel::Mid) => 0x07,
            AncMode::NoiseCancellation(AncLevel::Low) => 0x03,
            AncMode::NoiseCancellation(AncLevel::Adaptive) => 0x04,
            AncMode::Transparency => 0x01,
            AncMode::Off => 0x02,
        };

        let payload = [0x01, byte_val, 0x00];
        self.send_command(Command::SetAnc, &payload).await?;
        self.read_and_process_responses().await?;

        {
            let mut state = self.state.lock().await;
            state.anc_mode = mode;
        }
        self.emit_state().await;
        Ok(())
    }

    pub async fn set_ultra_bass(&self, enabled: bool, level: u8) -> Result<(), TransportError> {
        info!("Setting Ultra Bass: enabled={}, level={}", enabled, level);
        let enabled_byte = if enabled { 0x01 } else { 0x00 };
        let level_byte = level.min(5) * 2;

        self.send_command(Command::SetEnhancedBass, &[enabled_byte, level_byte]).await?;
        self.read_and_process_responses().await?;

        {
            let mut state = self.state.lock().await;
            state.eq.ultra_bass_enabled = enabled;
            state.eq.ultra_bass_level = level;
        }
        self.emit_state().await;
        Ok(())
    }

    pub async fn set_custom_eq(&self, bass: f32, mid: f32, treble: f32) -> Result<(), TransportError> {
        info!("Setting Custom 3-Band EQ: bass={}, mid={}, treble={}", bass, mid, treble);
        let levels = [bass, mid, treble];
        let max_val = bass.max(mid).max(treble);
        let preamp_gain = max_val / -1.0;

        let mut payload = vec![0x03]; // 3 bands
        let preamp_bytes = format_float_for_eq(preamp_gain, true);
        payload.extend_from_slice(&preamp_bytes);

        for &val in &levels {
            let b = format_float_for_eq(val, false);
            payload.extend_from_slice(&b);
            // Pad band descriptors
            payload.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        }

        self.send_command(Command::SetCustomEq, &payload).await?;
        self.read_and_process_responses().await?;

        {
            let mut state = self.state.lock().await;
            state.eq.custom_bass = bass;
            state.eq.custom_mid = mid;
            state.eq.custom_treble = treble;
            state.eq.preset = "Custom".into();
        }
        self.emit_state().await;
        Ok(())
    }

    pub async fn set_in_ear_detection(&self, enabled: bool) -> Result<(), TransportError> {
        let val = if enabled { 0x01 } else { 0x00 };
        self.send_command(Command::SetInEar, &[0x01, 0x01, val]).await?;
        self.read_and_process_responses().await?;

        {
            let mut state = self.state.lock().await;
            state.in_ear_detection = enabled;
        }
        self.emit_state().await;
        Ok(())
    }

    pub async fn set_low_latency(&self, enabled: bool) -> Result<(), TransportError> {
        let val = if enabled { 0x01 } else { 0x02 };
        self.send_command(Command::SetLatencyMode, &[val, 0x00]).await?;
        self.read_and_process_responses().await?;

        {
            let mut state = self.state.lock().await;
            state.low_latency_mode = enabled;
        }
        self.emit_state().await;
        Ok(())
    }

    pub async fn ring_earbuds(&self, is_left: bool, start_ring: bool) -> Result<(), TransportError> {
        let device_byte = if is_left { 0x02 } else { 0x03 };
        let action_byte = if start_ring { 0x01 } else { 0x00 };

        self.send_command(Command::RingBuds, &[device_byte, action_byte]).await?;
        Ok(())
    }

    async fn emit_state(&self) {
        let current = self.state.lock().await.clone();
        let _ = self.event_sender.send(current);
    }
}
