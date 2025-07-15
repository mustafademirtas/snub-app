use crate::modules::audio::macos_audio;
use crate::modules::errors::log_error;
use crate::modules::menu::emit_state_change;
use crate::modules::sound::macos_sound;
use crate::modules::tray;
use crate::modules::types::{MicrophoneState, Settings};
use std::sync::Mutex;

// In a real app, you would use a proper settings storage like a config file or database
// For now, we'll use a simple in-memory storage
static SETTINGS: Mutex<Settings> = Mutex::new(Settings { telemetry_enabled: false });

#[tauri::command]
pub fn get_microphone_state() -> Result<MicrophoneState, String> {
    macos_audio::get_microphone_state().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_microphone(app: tauri::AppHandle) -> Result<MicrophoneState, String> {
    // Get current state and toggle it
    let current_state = macos_audio::get_microphone_state().map_err(|e| e.to_string())?;
    let new_state = macos_audio::set_microphone_mute(!current_state.is_muted).map_err(|e| e.to_string())?;
    
    // Play sound based on new state
    if new_state.is_muted {
        if let Err(e) = macos_sound::play_mute_sound() {
            log_error("Failed to play mute sound", e);
        }
    } else {
        if let Err(e) = macos_sound::play_unmute_sound() {
            log_error("Failed to play unmute sound", e);
        }
    }
    
    // Update tray and emit event
    if let Err(e) = tray::update_tray_state(&app, new_state.is_muted) {
        log_error("Failed to update tray state", e);
    }
    emit_state_change(&app, &new_state);
    
    Ok(new_state)
}

#[tauri::command]
pub fn set_microphone_mute(app: tauri::AppHandle, mute: bool) -> Result<MicrophoneState, String> {
    let new_state = macos_audio::set_microphone_mute(mute).map_err(|e| e.to_string())?;
    
    // Play sound based on new state
    if new_state.is_muted {
        if let Err(e) = macos_sound::play_mute_sound() {
            log_error("Failed to play mute sound", e);
        }
    } else {
        if let Err(e) = macos_sound::play_unmute_sound() {
            log_error("Failed to play unmute sound", e);
        }
    }
    
    // Update tray and emit event
    if let Err(e) = tray::update_tray_state(&app, new_state.is_muted) {
        log_error("Failed to update tray state", e);
    }
    emit_state_change(&app, &new_state);
    
    Ok(new_state)
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    SETTINGS.lock()
        .map(|settings| settings.clone())
        .map_err(|e| format!("Failed to get settings: {}", e))
}

#[tauri::command]
pub fn set_telemetry_enabled(enabled: bool) -> Result<(), String> {
    SETTINGS.lock()
        .map(|mut settings| {
            settings.telemetry_enabled = enabled;
            println!("Telemetry {}", if enabled { "enabled" } else { "disabled" });
        })
        .map_err(|e| format!("Failed to set telemetry setting: {}", e))
}
