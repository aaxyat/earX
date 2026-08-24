use earx_lib::models::{
    get_device_model_from_serial, get_device_model_from_sku, parse_serial_number_payload, BaseId,
};

#[test]
fn test_sku_lookup_cmf_buds_pro_2() {
    let model = get_device_model_from_sku("79").expect("SKU 79 must exist");
    assert_eq!(model.name, "CMF Buds Pro 2");
    assert_eq!(model.base, BaseId::B172);
    assert_eq!(model.color_variant, "blue");
    assert!(model.is_anc_supported);
    assert!(model.is_ultra_bass_supported);
    assert!(model.is_listening_mode_device);
}

#[test]
fn test_sku_lookup_nothing_ear() {
    let model = get_device_model_from_sku("61").expect("SKU 61 must exist");
    assert_eq!(model.name, "Nothing Ear");
    assert_eq!(model.base, BaseId::B171);
    assert!(model.is_advanced_eq_supported);
}

#[test]
fn test_serial_parser_sh_prefix() {
    let serial = "SH247900123456"; // CMF Buds Pro 2 Blue (SKU 79)
    let model = get_device_model_from_serial(serial).expect("Must match serial");
    assert_eq!(model.name, "CMF Buds Pro 2");
    assert_eq!(model.base, BaseId::B172);
}

#[test]
fn test_serial_parser_ma_prefix() {
    let serial_stick = "MA00002200123";
    let model_stick = get_device_model_from_serial(serial_stick).expect("Must match Ear stick");
    assert_eq!(model_stick.base, BaseId::B157);

    let serial_open = "MA00002400123";
    let model_open = get_device_model_from_serial(serial_open).expect("Must match Ear open");
    assert_eq!(model_open.base, BaseId::B174);
}

#[test]
fn test_parse_serial_number_payload() {
    let payload = b"\x01\x00\x00\x00\x00\x00\x001,4,SH247900123456\n2,1,VAL2\n";
    let parsed = parse_serial_number_payload(payload);
    assert_eq!(parsed.as_deref(), Some("SH247900123456"));
}
