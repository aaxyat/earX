use super::mock::MockBluetoothTransport;
use super::trait_def::BluetoothTransport;

#[cfg(target_os = "windows")]
use super::windows_rfcomm::WindowsBluetoothTransport;

#[cfg(target_os = "macos")]
use super::macos_rfcomm::MacBluetoothTransport;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use super::serial_port::SerialPortTransport;

/// Creates the platform-appropriate Bluetooth transport or Mock transport for testing
pub fn create_platform_transport(use_mock: bool) -> Box<dyn BluetoothTransport> {
    if use_mock {
        return Box::new(MockBluetoothTransport::new());
    }

    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsBluetoothTransport::new())
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(MacBluetoothTransport::new())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Box::new(SerialPortTransport::new())
    }
}
