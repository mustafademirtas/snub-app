use crate::modules::audio::macos_audio;
use crate::modules::errors::log_error;
use crate::modules::menu;
use crate::modules::tray;
use crate::modules::types::{MicrophoneState, TRAY_ID, MAIN_WINDOW_ID};
use tauri::{tray::TrayIconBuilder, Manager};

/// Sets up the system tray
pub fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let initial_state = get_initial_microphone_state();
    let menu = tray::create_menu(app.handle(), initial_state.is_muted)?;
    let icon = tray::create_icon(initial_state.is_muted)?;

    let _tray = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .icon(icon)
        .tooltip("Snub")
        .on_tray_icon_event(|_tray, _event| {
            // Tray icon click events can be handled here if needed
        })
        .on_menu_event(|app, event| {
            menu::handle_menu_event(app, &event.id().0);
        })
        .build(app)?;

    // Initialize tray state
    if let Err(e) = tray::update_tray_state(app.handle(), initial_state.is_muted) {
        log_error("Failed to initialize tray state", e);
    }

    Ok(())
}

/// Sets up window event handlers
pub fn setup_window_handlers(app: &tauri::App) {
    if let Some(main_window) = app.get_webview_window(MAIN_WINDOW_ID) {
        let main_window_clone = main_window.clone();
        let _ = main_window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Prevent the window from closing and hide it instead
                api.prevent_close();
                let _ = main_window_clone.hide();
            }
        });
    }
}

/// Gets the initial microphone state with fallback
fn get_initial_microphone_state() -> MicrophoneState {
    macos_audio::get_microphone_state().unwrap_or_else(|e| {
        log_error("Failed to get initial microphone state", e);
        MicrophoneState::unmuted() // Default to unmuted
    })
}

#[cfg(target_os = "macos")]
pub fn hide_from_dock() {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory};
    
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
    }
}
