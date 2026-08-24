pub mod factory;
pub mod macos_rfcomm;
pub mod mock;
pub mod serial_port;
pub mod trait_def;
pub mod windows_rfcomm;

pub use factory::create_platform_transport;
pub use macos_rfcomm::MacBluetoothTransport;
pub use mock::{MockBluetoothTransport, MockDeviceState};
pub use serial_port::SerialPortTransport;
pub use trait_def::{BluetoothTransport, DiscoveredDevice, TransportError};
pub use windows_rfcomm::WindowsBluetoothTransport;
