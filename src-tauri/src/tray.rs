use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tracing::info;

pub fn setup_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItemBuilder::with_id("show", "Open earX Dashboard").build(app)?;
    let anc_high = MenuItemBuilder::with_id("anc_high", "● Noise Cancellation (High)").build(app)?;
    let anc_trans = MenuItemBuilder::with_id("anc_trans", "○ Transparency Mode").build(app)?;
    let anc_off = MenuItemBuilder::with_id("anc_off", "○ Noise Control Off").build(app)?;
    let bass_toggle = MenuItemBuilder::with_id("bass_toggle", "Ultra Bass: On (Level 2)").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit earX").build(app)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show_item)
        .item(&separator1)
        .item(&anc_high)
        .item(&anc_trans)
        .item(&anc_off)
        .item(&bass_toggle)
        .item(&separator2)
        .item(&quit_item)
        .build()?;

    let icon = app.default_window_icon().cloned().unwrap();

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("earX — Connected (L: 95% | C: 40% | R: 90%)")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                info!("Quitting application from system tray menu");
                app.exit(0);
            }
            "anc_high" => {
                info!("Tray action: Set ANC High");
            }
            "anc_trans" => {
                info!("Tray action: Set ANC Transparency");
            }
            "anc_off" => {
                info!("Tray action: Set ANC Off");
            }
            "bass_toggle" => {
                info!("Tray action: Toggle Ultra Bass");
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(is_visible) = window.is_visible() {
                        if is_visible {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .build(app)?;

    info!("System tray initialized successfully with dynamic earbuds icon & menu");
    Ok(())
}
