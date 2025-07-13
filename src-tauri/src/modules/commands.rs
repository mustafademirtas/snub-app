use crate::modules::audio::macos_audio;
use crate::modules::errors::log_error;
use crate::modules::menu::emit_state_change;
use crate::modules::tray;
use crate::modules::types::MicrophoneState;

#[tauri::command]
pub fn get_microphone_state() -> Result<MicrophoneState, String> {
    macos_audio::get_microphone_state().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_microphone(app: tauri::AppHandle) -> Result<MicrophoneState, String> {
    // Get current state and toggle it
    let current_state = macos_audio::get_microphone_state().map_err(|e| e.to_string())?;
    let new_state = macos_audio::set_microphone_mute(!current_state.is_muted).map_err(|e| e.to_string())?;
    
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
    
    // Update tray and emit event
    if let Err(e) = tray::update_tray_state(&app, new_state.is_muted) {
        log_error("Failed to update tray state", e);
    }
    emit_state_change(&app, &new_state);
    
    Ok(new_state)
}
