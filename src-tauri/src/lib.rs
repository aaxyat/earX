pub mod commands;
pub mod models;
pub mod protocol;
pub mod state;
pub mod transport;
pub mod tray;

use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, WindowEvent};
use tokio::time::sleep;
use tracing::{error, info};

use commands::{
    connect_device, disconnect_device, get_device_state, ring_earbuds, scan_devices, set_anc,
    set_custom_eq, set_in_ear, set_low_latency, set_ultra_bass, AppDeviceManager,
};
use state::DeviceManager;
use transport::create_platform_transport;
use tray::setup_system_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info,earx=debug")
        .try_init();

    info!("Starting earX desktop runtime with native Bluetooth drivers...");

    // Create native platform transport by default (Windows RFCOMM & macOS IOBluetooth)
    let transport = create_platform_transport(false);
    let manager: AppDeviceManager = Arc::new(DeviceManager::new(transport));

    let manager_for_events = Arc::clone(&manager);
    let manager_for_poll = Arc::clone(&manager);
    let manager_for_autoconnect = Arc::clone(&manager);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage(manager)
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let manager = manager_for_events;

            // Setup system tray
            if let Err(e) = setup_system_tray(&app_handle) {
                error!("Failed to initialize system tray: {}", e);
            }

            // Spawn state change listener to broadcast events to the frontend and update tray
            let app_for_broadcast = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let mut rx = manager.subscribe();
                while let Ok(state) = rx.recv().await {
                    let _ = app_for_broadcast.emit("device-state-changed", &state);

                    // Update tray tooltip if tray icon exists
                    if let Some(tray) = app_for_broadcast.tray_by_id("main-tray") {
                        if state.is_connected {
                            let left_str = state.battery.left.map(|l| format!("{}%", l)).unwrap_or_else(|| "--".into());
                            let case_str = state.battery.case.map(|c| format!("{}%", c)).unwrap_or_else(|| "--".into());
                            let right_str = state.battery.right.map(|r| format!("{}%", r)).unwrap_or_else(|| "--".into());
                            let tooltip = format!("{} — L: {} | C: {} | R: {}", state.device_name, left_str, case_str, right_str);
                            let _ = tray.set_tooltip(Some(tooltip));
                        } else {
                            let _ = tray.set_tooltip(Some(String::from("earX — Disconnected")));
                        }
                    }
                }
            });

            // Auto-connect task
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_millis(500)).await;
                info!("Attempting auto-discovery for paired Nothing/CMF earbuds...");
                let _ = manager_for_autoconnect.auto_connect().await;
            });

            // Periodic telemetry polling loop (every 5 seconds)
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_secs(5)).await;
                    let _ = manager_for_poll.poll_battery().await;
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Minimize to tray on close request instead of quitting
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_device_state,
            scan_devices,
            connect_device,
            disconnect_device,
            set_anc,
            set_ultra_bass,
            set_custom_eq,
            set_in_ear,
            set_low_latency,
            ring_earbuds,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
