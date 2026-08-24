pub mod device_info;
pub mod sku_map;

pub use device_info::{BaseId, DeviceModelInfo};
pub use sku_map::{get_device_model_from_serial, get_device_model_from_sku, parse_serial_number_payload};
