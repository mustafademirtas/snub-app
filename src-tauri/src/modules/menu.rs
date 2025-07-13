use crate::modules::audio::macos_audio;
use crate::modules::errors::log_error;
use crate::modules::tray;
use crate::modules::types::{MENU_TOGGLE_MIC, MENU_SHOW_WINDOW, MENU_QUIT, MAIN_WINDOW_ID, MicrophoneState};
use tauri::{Emitter, Manager, Runtime};

/// Emits microphone state change event to the frontend
pub fn emit_state_change<R: Runtime>(app: &tauri::AppHandle<R>, state: &MicrophoneState) {
    if let Err(e) = app.emit("microphone-state-changed", state) {
        eprintln!("Failed to emit microphone state event: {}", e);
    }
}

/// Handles tray menu item clicks
pub fn handle_menu_event(app: &tauri::AppHandle, event_id: &str) {
    match event_id {
        MENU_TOGGLE_MIC => handle_toggle_microphone(app),
        MENU_SHOW_WINDOW => handle_show_window(app),
        MENU_QUIT => app.exit(0),
        _ => {}
    }
}

/// Handles microphone toggle from menu
fn handle_toggle_microphone(app: &tauri::AppHandle) {
    match macos_audio::get_microphone_state() {
        Ok(current_state) => {
            match macos_audio::set_microphone_mute(!current_state.is_muted) {
                Ok(new_state) => {
                    if let Err(e) = tray::update_tray_state(app, new_state.is_muted) {
                        log_error("Failed to update tray state", e);
                    }
                    emit_state_change(app, &new_state);
                }
                Err(e) => log_error("Failed to set microphone mute", e),
            }
        }
        Err(e) => log_error("Failed to get microphone state", e),
    }
}

/// Handles show window request
fn handle_show_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_ID) {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
