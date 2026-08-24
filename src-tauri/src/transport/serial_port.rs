use async_trait::async_trait;
use serialport::{SerialPort, SerialPortType};
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use super::trait_def::{BluetoothTransport, DiscoveredDevice, TransportError};

pub struct SerialPortTransport {
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    port_name: String,
    connected: bool,
}

impl SerialPortTransport {
    pub fn new() -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
            port_name: String::new(),
            connected: false,
        }
    }
}

#[async_trait]
impl BluetoothTransport for SerialPortTransport {
    async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        let ports = serialport::available_ports().map_err(|e| TransportError::Io(e.to_string()))?;
        let mut devices = Vec::new();

        for p in ports {
            let is_bt = match &p.port_type {
                SerialPortType::BluetoothPort => true,
                _ => p.port_name.contains("Bluetooth") || p.port_name.starts_with("COM"),
            };

            if is_bt {
                devices.push(DiscoveredDevice {
                    name: format!("Bluetooth Device ({})", p.port_name),
                    address: p.port_name.clone(),
                    is_connected: self.connected && self.port_name == p.port_name,
                });
            }
        }

        Ok(devices)
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Connecting via Serial Port transport to {}", address);

        let builder = serialport::new(address, 9600)
            .timeout(Duration::from_millis(500))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One);

        let port_instance = builder.open().map_err(|e| {
            warn!("Failed to open serial port {}: {}", address, e);
            TransportError::ConnectionFailed(format!("Failed to open port {}: {}", address, e))
        })?;

        let mut lock = self.port.lock().await;
        *lock = Some(port_instance);
        self.port_name = address.to_string();
        self.connected = true;

        info!("Successfully connected to serial port {}", address);
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    async fn send(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        let mut lock = self.port.lock().await;
        if let Some(port) = lock.as_mut() {
            debug!("Sending {} bytes over serial port", frame.len());
            port.write_all(frame).map_err(|e| TransportError::Io(e.to_string()))?;
            port.flush().map_err(|e| TransportError::Io(e.to_string()))?;
            Ok(())
        } else {
            Err(TransportError::Disconnected("Serial port not open".into()))
        }
    }

    async fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        let mut lock = self.port.lock().await;
        if let Some(port) = lock.as_mut() {
            let mut buffer = [0u8; 1024];
            match port.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    debug!("Received {} bytes from serial port", n);
                    Ok(buffer[0..n].to_vec())
                }
                Ok(_) => Ok(Vec::new()),
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(Vec::new()),
                Err(e) => Err(TransportError::Io(e.to_string())),
            }
        } else {
            Err(TransportError::Disconnected("Serial port not open".into()))
        }
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        let mut lock = self.port.lock().await;
        *lock = None;
        self.connected = false;
        self.port_name.clear();
        info!("Disconnected serial port transport");
        Ok(())
    }
}
