use serde::{Deserialize, Serialize};
use crate::models::DeviceModelInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatteryTelemetry {
    pub left: Option<u8>,
    pub right: Option<u8>,
    pub case: Option<u8>,
    pub is_charging_left: bool,
    pub is_charging_right: bool,
    pub is_charging_case: bool,
}

impl Default for BatteryTelemetry {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            case: None,
            is_charging_left: false,
            is_charging_right: false,
            is_charging_case: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AncLevel {
    Low,
    Mid,
    High,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "level")]
pub enum AncMode {
    Off,
    Transparency,
    NoiseCancellation(AncLevel),
}

impl Default for AncMode {
    fn default() -> Self {
        AncMode::NoiseCancellation(AncLevel::High)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EqSettings {
    pub preset: String, // "Balanced", "More Bass", "More Treble", "Voice", "Rock", "Pop", "Custom"
    pub custom_bass: f32,
    pub custom_mid: f32,
    pub custom_treble: f32,
    pub ultra_bass_enabled: bool,
    pub ultra_bass_level: u8, // 0 .. 5
}

impl Default for EqSettings {
    fn default() -> Self {
        Self {
            preset: "Rock".into(),
            custom_bass: 0.0,
            custom_mid: 0.0,
            custom_treble: 0.0,
            ultra_bass_enabled: true,
            ultra_bass_level: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceState {
    pub is_connected: bool,
    pub device_name: String,
    pub address: String,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub model: Option<DeviceModelInfo>,
    pub battery: BatteryTelemetry,
    pub anc_mode: AncMode,
    pub eq: EqSettings,
    pub in_ear_detection: bool,
    pub low_latency_mode: bool,
    pub spatial_audio: bool,
    pub dual_connection: bool,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            is_connected: false,
            device_name: "CMF Buds 2 Plus".into(),
            address: "".into(),
            serial_number: None,
            firmware_version: None,
            model: None,
            battery: BatteryTelemetry::default(),
            anc_mode: AncMode::default(),
            eq: EqSettings::default(),
            in_ear_detection: true,
            low_latency_mode: false,
            spatial_audio: false,
            dual_connection: true,
        }
    }
}
