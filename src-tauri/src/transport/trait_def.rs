use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub name: String,
    pub address: String,
    pub is_connected: bool,
}

#[derive(Error, Debug, Clone)]
pub enum TransportError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Disconnected: {0}")]
    Disconnected(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

#[async_trait]
pub trait BluetoothTransport: Send + Sync {
    /// Discovers nearby paired or available Nothing / CMF Bluetooth devices
    async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError>;

    /// Establishes an RFCOMM / SPP stream connection to the specified device address
    async fn connect(&mut self, address: &str) -> Result<(), TransportError>;

    /// Returns true if currently connected
    async fn is_connected(&self) -> bool;

    /// Sends a raw binary packet frame
    async fn send(&mut self, frame: &[u8]) -> Result<(), TransportError>;

    /// Receives the next incoming raw packet frame from the stream
    async fn receive(&mut self) -> Result<Vec<u8>, TransportError>;

    /// Closes the connection
    async fn disconnect(&mut self) -> Result<(), TransportError>;
}
