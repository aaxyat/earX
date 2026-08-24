use async_trait::async_trait;
use tracing::info;

use super::serial_port::SerialPortTransport;
use super::trait_def::{BluetoothTransport, DiscoveredDevice, TransportError};

pub struct WindowsBluetoothTransport {
    inner: SerialPortTransport,
}

impl WindowsBluetoothTransport {
    pub fn new() -> Self {
        Self {
            inner: SerialPortTransport::new(),
        }
    }
}

#[async_trait]
impl BluetoothTransport for WindowsBluetoothTransport {
    async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        info!("Scanning for Bluetooth devices on Windows...");
        self.inner.scan_devices().await
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Windows Bluetooth connecting to: {}", address);
        self.inner.connect(address).await
    }

    async fn is_connected(&self) -> bool {
        self.inner.is_connected().await
    }

    async fn send(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        self.inner.send(frame).await
    }

    async fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        self.inner.receive().await
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.inner.disconnect().await
    }
}
