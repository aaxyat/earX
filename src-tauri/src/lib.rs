pub mod commands;
pub mod models;
pub mod protocol;
pub mod state;
pub mod transport;

use std::sync::Arc;
use tauri::Emitter;
use tracing::{error, info};

use commands::{
    connect_device, disconnect_device, get_device_state, ring_earbuds, scan_devices, set_anc,
    set_custom_eq, set_in_ear, set_low_latency, set_ultra_bass, AppDeviceManager,
};
use state::DeviceManager;
use transport::create_platform_transport;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info,earx=debug")
        .try_init();

    info!("Starting earX desktop runtime...");

    // Create transport (defaults to Mock transport if no hardware, or platform Bluetooth transport)
    let transport = create_platform_transport(true); // default with mock capable fallback
    let manager: AppDeviceManager = Arc::new(DeviceManager::new(transport));

    let manager_for_events = Arc::clone(&manager);

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

            // Spawn state change listener to broadcast events to the frontend
            tauri::async_runtime::spawn(async move {
                let mut rx = manager.subscribe();
                while let Ok(state) = rx.recv().await {
                    if let Err(e) = app_handle.emit("device-state-changed", &state) {
                        error!("Failed to emit device-state-changed event: {}", e);
                    }
                }
            });

            Ok(())
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
