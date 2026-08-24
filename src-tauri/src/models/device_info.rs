use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseId {
    B181, // Nothing Ear (1)
    B157, // Nothing Ear (stick)
    B155, // Nothing Ear (2)
    B163, // CMF Buds Pro
    B164, // CMF Neckband Pro
    B168, // CMF Buds
    B171, // Nothing Ear (2024)
    B162, // Nothing Ear (a)
    B172, // CMF Buds Pro 2 / CMF Buds 2 Plus
    B174, // Nothing Ear (open)
    Unknown,
}

impl BaseId {
    pub fn as_str(&self) -> &'static str {
        match self {
            BaseId::B181 => "B181",
            BaseId::B157 => "B157",
            BaseId::B155 => "B155",
            BaseId::B163 => "B163",
            BaseId::B164 => "B164",
            BaseId::B168 => "B168",
            BaseId::B171 => "B171",
            BaseId::B162 => "B162",
            BaseId::B172 => "B172",
            BaseId::B174 => "B174",
            BaseId::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceModelInfo {
    pub name: String,
    pub base: BaseId,
    pub code_name: String,
    pub color_variant: String,
    pub is_anc_supported: bool,
    pub is_ultra_bass_supported: bool,
    pub is_advanced_eq_supported: bool,
    pub is_listening_mode_device: bool,
    pub is_fit_test_supported: bool,
    pub is_case_led_supported: bool,
    pub left_image: String,
    pub right_image: String,
    pub case_image: String,
    pub duo_image: String,
}
