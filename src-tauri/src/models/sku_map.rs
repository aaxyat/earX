use super::device_info::{BaseId, DeviceModelInfo};

/// Parses a hardware serial number from the `0x4006` raw payload
pub fn parse_serial_number_payload(payload: &[u8]) -> Option<String> {
    if payload.len() < 7 {
        return None;
    }

    let text_part = String::from_utf8_lossy(&payload[7..]);
    for line in text_part.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let config_type: Result<u32, _> = parts[1].trim().parse();
            let val = parts[2].trim();
            if config_type == Ok(4) && !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

/// Identifies the device model info from a serial number string
pub fn get_device_model_from_serial(serial: &str) -> Option<DeviceModelInfo> {
    if serial.is_empty() {
        return None;
    }

    if serial == "12345678901234567" {
        return get_device_model_from_sku("01");
    }

    if serial.starts_with("MA") {
        if serial.len() >= 8 {
            let year = &serial[6..8];
            if year == "22" || year == "23" {
                return get_device_model_from_sku("14"); // Ear (stick)
            } else if year == "24" {
                return get_device_model_from_sku("11200005"); // Ear (open)
            }
        }
        return get_device_model_from_sku("14");
    }

    if (serial.starts_with("SH") || serial.starts_with("13")) && serial.len() >= 6 {
        let sku = &serial[4..6];
        return get_device_model_from_sku(sku);
    }

    None
}

/// Lookup model definition by SKU code
pub fn get_device_model_from_sku(sku: &str) -> Option<DeviceModelInfo> {
    match sku {
        // Nothing Ear (1)
        "01" | "03" | "07" => Some(DeviceModelInfo {
            name: "Nothing Ear (1)".into(),
            base: BaseId::B181,
            code_name: "one".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: true,
            left_image: "/assets/ear_one_white_left.webp".into(),
            right_image: "/assets/ear_one_white_right.webp".into(),
            case_image: "/assets/ear_one_white_case.webp".into(),
            duo_image: "/assets/ear_one_white_duo.webp".into(),
        }),
        "02" | "04" | "06" | "08" | "10" => Some(DeviceModelInfo {
            name: "Nothing Ear (1)".into(),
            base: BaseId::B181,
            code_name: "one".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: true,
            left_image: "/assets/ear_one_black_left.webp".into(),
            right_image: "/assets/ear_one_black_right.webp".into(),
            case_image: "/assets/ear_one_black_case.webp".into(),
            duo_image: "/assets/ear_one_black_duo.webp".into(),
        }),

        // Nothing Ear (stick)
        "14" | "15" | "16" => Some(DeviceModelInfo {
            name: "Nothing Ear (stick)".into(),
            base: BaseId::B157,
            code_name: "sticks".into(),
            color_variant: "white".into(),
            is_anc_supported: false,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/ear_stick_left.webp".into(),
            right_image: "/assets/ear_stick_right.webp".into(),
            case_image: "/assets/ear_stick_case_none.webp".into(),
            duo_image: "/assets/ear_stick_white_duo.webp".into(),
        }),

        // Nothing Ear (2)
        "17" | "18" | "19" => Some(DeviceModelInfo {
            name: "Nothing Ear (2)".into(),
            base: BaseId::B155,
            code_name: "two".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: true,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_two_white_left.webp".into(),
            right_image: "/assets/ear_two_white_right.webp".into(),
            case_image: "/assets/ear_two_white_case.webp".into(),
            duo_image: "/assets/ear_two_white_duo.webp".into(),
        }),
        "27" | "28" | "29" => Some(DeviceModelInfo {
            name: "Nothing Ear (2)".into(),
            base: BaseId::B155,
            code_name: "two".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: true,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_two_black_left.webp".into(),
            right_image: "/assets/ear_two_black_right.webp".into(),
            case_image: "/assets/ear_two_black_case.webp".into(),
            duo_image: "/assets/ear_two_black_duo.webp".into(),
        }),

        // CMF Buds Pro
        "30" | "31" => Some(DeviceModelInfo {
            name: "CMF Buds Pro".into(),
            base: BaseId::B163,
            code_name: "corsola".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/ear_corsola_black_left.webp".into(),
            right_image: "/assets/ear_corsola_black_right.webp".into(),
            case_image: "/assets/ear_corsola_black_case.webp".into(),
            duo_image: "".into(),
        }),
        "32" | "33" => Some(DeviceModelInfo {
            name: "CMF Buds Pro".into(),
            base: BaseId::B163,
            code_name: "corsola".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/ear_corsola_white_left.webp".into(),
            right_image: "/assets/ear_corsola_white_right.webp".into(),
            case_image: "/assets/ear_corsola_white_case.webp".into(),
            duo_image: "".into(),
        }),
        "34" | "35" => Some(DeviceModelInfo {
            name: "CMF Buds Pro".into(),
            base: BaseId::B163,
            code_name: "corsola".into(),
            color_variant: "orange".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: false,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/ear_corsola_orange_left.webp".into(),
            right_image: "/assets/ear_corsola_orange_right.webp".into(),
            case_image: "/assets/ear_corsola_orange_case.webp".into(),
            duo_image: "".into(),
        }),

        // CMF Buds
        "54" | "55" => Some(DeviceModelInfo {
            name: "CMF Buds".into(),
            base: BaseId::B168,
            code_name: "donphan".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/donphan_black_left.webp".into(),
            right_image: "/assets/donphan_black_right.webp".into(),
            case_image: "/assets/donphan_black_case.webp".into(),
            duo_image: "".into(),
        }),
        "56" | "57" => Some(DeviceModelInfo {
            name: "CMF Buds".into(),
            base: BaseId::B168,
            code_name: "donphan".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/donphan_white_left.webp".into(),
            right_image: "/assets/donphan_white_right.webp".into(),
            case_image: "/assets/donphan_white_case.webp".into(),
            duo_image: "".into(),
        }),
        "58" | "59" => Some(DeviceModelInfo {
            name: "CMF Buds".into(),
            base: BaseId::B168,
            code_name: "donphan".into(),
            color_variant: "orange".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/donphan_orange_left.webp".into(),
            right_image: "/assets/donphan_orange_right.webp".into(),
            case_image: "/assets/donphan_orange_case.webp".into(),
            duo_image: "".into(),
        }),

        // Nothing Ear (2024)
        "61" | "69" | "74" => Some(DeviceModelInfo {
            name: "Nothing Ear".into(),
            base: BaseId::B171,
            code_name: "entei".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: true,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_twos_black_left.webp".into(),
            right_image: "/assets/ear_twos_black_right.webp".into(),
            case_image: "/assets/ear_twos_black_case.webp".into(),
            duo_image: "".into(),
        }),
        "62" | "70" | "75" => Some(DeviceModelInfo {
            name: "Nothing Ear".into(),
            base: BaseId::B171,
            code_name: "entei".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: true,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_twos_white_left.webp".into(),
            right_image: "/assets/ear_twos_white_right.webp".into(),
            case_image: "/assets/ear_twos_white_case.webp".into(),
            duo_image: "".into(),
        }),

        // Nothing Ear (a)
        "63" | "66" | "71" => Some(DeviceModelInfo {
            name: "Nothing Ear (a)".into(),
            base: BaseId::B162,
            code_name: "cleffa".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_color_black_left.webp".into(),
            right_image: "/assets/ear_color_black_right.webp".into(),
            case_image: "/assets/ear_color_black_case.webp".into(),
            duo_image: "".into(),
        }),
        "64" | "67" | "72" => Some(DeviceModelInfo {
            name: "Nothing Ear (a)".into(),
            base: BaseId::B162,
            code_name: "cleffa".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_color_white_left.webp".into(),
            right_image: "/assets/ear_color_white_right.webp".into(),
            case_image: "/assets/ear_color_white_case.webp".into(),
            duo_image: "".into(),
        }),
        "65" | "68" | "73" => Some(DeviceModelInfo {
            name: "Nothing Ear (a)".into(),
            base: BaseId::B162,
            code_name: "cleffa".into(),
            color_variant: "yellow".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/ear_color_yellow_left.webp".into(),
            right_image: "/assets/ear_color_yellow_right.webp".into(),
            case_image: "/assets/ear_color_yellow_case.webp".into(),
            duo_image: "".into(),
        }),

        // CMF Buds Pro 2 / Buds 2 Plus
        "76" | "83" => Some(DeviceModelInfo {
            name: "CMF Buds Pro 2".into(),
            base: BaseId::B172,
            code_name: "espeon".into(),
            color_variant: "black".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/espeon_black_left.webp".into(),
            right_image: "/assets/espeon_black_right.webp".into(),
            case_image: "/assets/espeon_black_case.webp".into(),
            duo_image: "".into(),
        }),
        "77" | "82" => Some(DeviceModelInfo {
            name: "CMF Buds Pro 2".into(),
            base: BaseId::B172,
            code_name: "espeon".into(),
            color_variant: "white".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/espeon_white_left.webp".into(),
            right_image: "/assets/espeon_white_right.webp".into(),
            case_image: "/assets/espeon_white_case.webp".into(),
            duo_image: "".into(),
        }),
        "78" | "81" => Some(DeviceModelInfo {
            name: "CMF Buds Pro 2".into(),
            base: BaseId::B172,
            code_name: "espeon".into(),
            color_variant: "orange".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/espeon_orange_left.webp".into(),
            right_image: "/assets/espeon_orange_right.webp".into(),
            case_image: "/assets/espeon_orange_case.webp".into(),
            duo_image: "".into(),
        }),
        "79" | "80" => Some(DeviceModelInfo {
            name: "CMF Buds Pro 2".into(),
            base: BaseId::B172,
            code_name: "espeon".into(),
            color_variant: "blue".into(),
            is_anc_supported: true,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: true,
            is_fit_test_supported: true,
            is_case_led_supported: false,
            left_image: "/assets/espeon_blue_left.webp".into(),
            right_image: "/assets/espeon_blue_right.webp".into(),
            case_image: "/assets/espeon_blue_case.webp".into(),
            duo_image: "".into(),
        }),

        // Nothing Ear (open)
        "11200005" => Some(DeviceModelInfo {
            name: "Nothing Ear (open)".into(),
            base: BaseId::B174,
            code_name: "flaaffy".into(),
            color_variant: "white".into(),
            is_anc_supported: false,
            is_ultra_bass_supported: true,
            is_advanced_eq_supported: false,
            is_listening_mode_device: false,
            is_fit_test_supported: false,
            is_case_led_supported: false,
            left_image: "/assets/flaffy_white_left.webp".into(),
            right_image: "/assets/flaffy_white_right.webp".into(),
            case_image: "/assets/flaffy_white_case.webp".into(),
            duo_image: "".into(),
        }),

        _ => None,
    }
}
