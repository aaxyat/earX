use std::sync::Arc;
use tauri::State;

use crate::state::{AncMode, DeviceManager, DeviceState};
use crate::transport::DiscoveredDevice;

pub type AppDeviceManager = Arc<DeviceManager>;

#[tauri::command]
pub async fn get_device_state(manager: State<'_, AppDeviceManager>) -> Result<DeviceState, String> {
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn scan_devices(
    manager: State<'_, AppDeviceManager>,
) -> Result<Vec<DiscoveredDevice>, String> {
    manager
        .scan_devices()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_device(
    address: String,
    name: Option<String>,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .connect(&address, name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn disconnect_device(manager: State<'_, AppDeviceManager>) -> Result<(), String> {
    manager
        .disconnect()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_anc(
    mode: AncMode,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .set_anc_mode(mode)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn set_ultra_bass(
    enabled: bool,
    level: u8,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .set_ultra_bass(enabled, level)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn set_custom_eq(
    bass: f32,
    mid: f32,
    treble: f32,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .set_custom_eq(bass, mid, treble)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn set_in_ear(
    enabled: bool,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .set_in_ear_detection(enabled)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn set_low_latency(
    enabled: bool,
    manager: State<'_, AppDeviceManager>,
) -> Result<DeviceState, String> {
    manager
        .set_low_latency(enabled)
        .await
        .map_err(|e| e.to_string())?;
    Ok(manager.get_state().await)
}

#[tauri::command]
pub async fn ring_earbuds(
    is_left: bool,
    start_ring: bool,
    manager: State<'_, AppDeviceManager>,
) -> Result<(), String> {
    manager
        .ring_earbuds(is_left, start_ring)
        .await
        .map_err(|e| e.to_string())
}
