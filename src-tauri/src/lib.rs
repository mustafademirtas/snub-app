//! # Snub - macOS Microphone Control Application
//! 
//! This is a Tauri-based application that provides a simple interface to control
//! the system microphone on macOS. The app features a system tray icon that changes
//! based on the microphone state and provides menu options for quick control.
//! 
//! ## Features
//! - System tray integration with dynamic icons
//! - One-click microphone mute/unmute
//! - Visual feedback for microphone state
//! - Minimal, non-intrusive UI
//! 
//! ## Architecture
//! The application is structured into several modules:
//! - `audio`: Handles macOS-specific audio control via AppleScript
//! - `tray`: Manages system tray icon and menu functionality  
//! - `menu`: Processes user interactions with tray menu items
//! - `setup`: Handles application initialization and setup
//! - `commands`: Tauri command handlers for frontend communication
//! - `types`: Data structures and constants
//! - `errors`: Error handling and logging

mod modules;

use modules::commands::{get_microphone_state, set_microphone_mute, toggle_microphone};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            modules::setup::hide_from_dock();
            
            modules::setup::setup_tray(app)?;
            modules::setup::setup_window_handlers(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_microphone_state,
            toggle_microphone,
            set_microphone_mute
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
