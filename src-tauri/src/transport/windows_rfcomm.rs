use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use super::serial_port::SerialPortTransport;
use super::trait_def::{BluetoothTransport, DiscoveredDevice, TransportError};

#[cfg(windows)]
use windows::{
    core::GUID,
    Win32::Devices::Bluetooth::{
        BluetoothFindFirstDevice, BluetoothFindNextDevice, BLUETOOTH_DEVICE_INFO,
        BLUETOOTH_DEVICE_SEARCH_PARAMS,
    },
    Win32::Foundation::{CloseHandle, HANDLE},
    Win32::Networking::WinSock::{
        closesocket, connect, recv, send, socket, WSACleanup, WSAGetLastError, WSAStartup,
        SEND_RECV_FLAGS, SOCKADDR, SOCK_STREAM, SOCKET, WSADATA,
    },
};

const AF_BTH_VAL: u16 = 32;
const BTHPROTO_RFCOMM_VAL: i32 = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
struct SockAddrBth {
    addressFamily: u16,
    btAddr: u64,
    serviceClassId: GUID,
    port: u32,
}

pub struct WindowsBluetoothTransport {
    #[cfg(windows)]
    socket_handle: Arc<Mutex<Option<SOCKET>>>,
    serial_fallback: SerialPortTransport,
    device_address: String,
    connected: bool,
    is_socket: bool,
}

impl WindowsBluetoothTransport {
    pub fn new() -> Self {
        #[cfg(windows)]
        {
            let mut wsa_data = WSADATA::default();
            unsafe {
                let _ = WSAStartup(0x0202, &mut wsa_data);
            }
        }

        Self {
            #[cfg(windows)]
            socket_handle: Arc::new(Mutex::new(None)),
            serial_fallback: SerialPortTransport::new(),
            device_address: String::new(),
            connected: false,
            is_socket: false,
        }
    }

    #[cfg(windows)]
    fn parse_mac_address(address_str: &str) -> Option<u64> {
        let clean = address_str.replace([':', '-', ' '], "");
        if clean.len() == 12 {
            u64::from_str_radix(&clean, 16).ok()
        } else {
            None
        }
    }

    #[cfg(windows)]
    fn format_mac_address(bth_addr: u64) -> String {
        let bytes = bth_addr.to_be_bytes();
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
        )
    }
}

impl Drop for WindowsBluetoothTransport {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            if let Ok(mut lock) = self.socket_handle.try_lock() {
                if let Some(sock) = lock.take() {
                    unsafe {
                        let _ = closesocket(sock);
                    }
                }
            }
            unsafe {
                let _ = WSACleanup();
            }
        }
    }
}

#[async_trait]
impl BluetoothTransport for WindowsBluetoothTransport {
    async fn scan_devices(&self) -> Result<Vec<DiscoveredDevice>, TransportError> {
        info!("Scanning for paired Nothing & CMF Bluetooth devices on Windows...");
        let mut devices = Vec::new();

        #[cfg(windows)]
        {
            unsafe {
                let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
                    fReturnAuthenticated: true.into(),
                    fReturnRemembered: true.into(),
                    fReturnUnknown: false.into(),
                    fReturnConnected: true.into(),
                    fIssueInquiry: false.into(),
                    cTimeoutMultiplier: 2,
                    hRadio: HANDLE::default(),
                };

                let mut device_info = BLUETOOTH_DEVICE_INFO {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
                    ..Default::default()
                };

                if let Ok(handle) = BluetoothFindFirstDevice(&search_params, &mut device_info) {
                    loop {
                        let name_end = device_info
                            .szName
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(device_info.szName.len());
                        let name = String::from_utf16_lossy(&device_info.szName[..name_end]);
                        let mac_str = Self::format_mac_address(device_info.Address.Anonymous.ullLong);

                        debug!("Found paired Windows Bluetooth device: {} ({})", name, mac_str);

                        let is_target = name.contains("Nothing")
                            || name.contains("CMF")
                            || name.contains("Buds")
                            || name.contains("Ear");

                        let dev = DiscoveredDevice {
                            name: name.clone(),
                            address: mac_str.clone(),
                            is_connected: device_info.fConnected.as_bool(),
                        };

                        if is_target {
                            devices.insert(0, dev);
                        } else {
                            devices.push(dev);
                        }

                        device_info = BLUETOOTH_DEVICE_INFO {
                            dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
                            ..Default::default()
                        };

                        if BluetoothFindNextDevice(handle, &mut device_info).is_err() {
                            break;
                        }
                    }
                    let _ = CloseHandle(HANDLE(handle.0));
                }
            }
        }

        // Also check virtual serial ports
        if let Ok(serial_devices) = self.serial_fallback.scan_devices().await {
            for sd in serial_devices {
                if !devices.iter().any(|d| d.address == sd.address) {
                    devices.push(sd);
                }
            }
        }

        info!("Discovered {} Bluetooth device(s)", devices.len());
        Ok(devices)
    }

    async fn connect(&mut self, address: &str) -> Result<(), TransportError> {
        info!("Attempting connection to Bluetooth device {}", address);

        #[cfg(windows)]
        if let Some(mac_u64) = Self::parse_mac_address(address) {
            info!("Opening native Winsock AF_BTH RFCOMM socket to {}", address);

            // Nothing / CMF SPP UUID: aeac4a03-dff5-498f-843a-34487cf133eb
            let spp_guid = GUID::from_u128(0xaeac4a03_dff5_498f_843a_34487cf133eb);

            if let Ok(sock) = unsafe { socket(AF_BTH_VAL as i32, SOCK_STREAM, BTHPROTO_RFCOMM_VAL) } {
                if sock != SOCKET::default() {
                    let sockaddr = SockAddrBth {
                        addressFamily: AF_BTH_VAL,
                        btAddr: mac_u64,
                        serviceClassId: spp_guid,
                        port: 0, // auto-bind RFCOMM channel
                    };

                    let connect_res = unsafe {
                        connect(
                            sock,
                            &sockaddr as *const _ as *const SOCKADDR,
                            std::mem::size_of::<SockAddrBth>() as i32,
                        )
                    };

                    if connect_res == 0 {
                        info!("Successfully connected native Winsock RFCOMM socket to {}", address);
                        let mut lock = self.socket_handle.lock().await;
                        *lock = Some(sock);
                        self.device_address = address.to_string();
                        self.connected = true;
                        self.is_socket = true;
                        return Ok(());
                    } else {
                        let err = unsafe { WSAGetLastError() };
                        warn!("Winsock RFCOMM connect returned error {:?}, attempting COM fallback", err);
                        unsafe {
                            let _ = closesocket(sock);
                        }
                    }
                }
            }
        }

        // Fallback to Serial COM port
        info!("Attempting SerialPort fallback for {}", address);
        self.serial_fallback.connect(address).await?;
        self.device_address = address.to_string();
        self.connected = true;
        self.is_socket = false;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    async fn send(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::Disconnected("Device not connected".into()));
        }

        if self.is_socket {
            #[cfg(windows)]
            {
                let lock = self.socket_handle.lock().await;
                if let Some(sock) = *lock {
                    let res = unsafe {
                        send(sock, frame, SEND_RECV_FLAGS(0))
                    };
                    if res < 0 {
                        let err = unsafe { WSAGetLastError() };
                        return Err(TransportError::Io(format!("Winsock send error: {:?}", err)));
                    }
                    return Ok(());
                }
            }
            Err(TransportError::Disconnected("Socket not available".into()))
        } else {
            self.serial_fallback.send(frame).await
        }
    }

    async fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        if !self.connected {
            return Err(TransportError::Disconnected("Device not connected".into()));
        }

        if self.is_socket {
            #[cfg(windows)]
            {
                let lock = self.socket_handle.lock().await;
                if let Some(sock) = *lock {
                    let mut buffer = [0u8; 2048];
                    let res = unsafe {
                        recv(sock, &mut buffer, SEND_RECV_FLAGS(0))
                    };

                    if res > 0 {
                        return Ok(buffer[0..res as usize].to_vec());
                    } else if res == 0 {
                        return Ok(Vec::new());
                    } else {
                        let err = unsafe { WSAGetLastError() };
                        // WSAEWOULDBLOCK / timeout
                        if err.0 == 10035 || err.0 == 10060 {
                            return Ok(Vec::new());
                        }
                        return Err(TransportError::Io(format!("Winsock recv error: {:?}", err)));
                    }
                }
            }
            Err(TransportError::Disconnected("Socket not available".into()))
        } else {
            self.serial_fallback.receive().await
        }
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        info!("Disconnecting Windows Bluetooth transport");
        if self.is_socket {
            #[cfg(windows)]
            {
                let mut lock = self.socket_handle.lock().await;
                if let Some(sock) = lock.take() {
                    unsafe {
                        let _ = closesocket(sock);
                    }
                }
            }
        } else {
            let _ = self.serial_fallback.disconnect().await;
        }

        self.connected = false;
        self.device_address.clear();
        Ok(())
    }
}
