use serde::{Deserialize, Serialize};

// Constants for better maintainability
pub const DEFAULT_UNMUTED_VOLUME: &str = "50";
pub const MUTED_VOLUME: &str = "0";
pub const TRAY_ID: &str = "main";
pub const MAIN_WINDOW_ID: &str = "main";
pub const SETTINGS_WINDOW_ID: &str = "settings";

// Menu item IDs
pub const MENU_TOGGLE_MIC: &str = "toggle_microphone";
pub const MENU_SHOW_WINDOW: &str = "show_window";
pub const MENU_SETTINGS: &str = "settings";
pub const MENU_QUIT: &str = "quit";

// Tray tooltips
pub const TOOLTIP_MUTED: &str = "Microphone: Muted (Click to unmute)";
pub const TOOLTIP_ACTIVE: &str = "Microphone: Active (Click to mute)";

/// Represents the current state of the microphone
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MicrophoneState {
    pub is_muted: bool,
}

impl MicrophoneState {
    pub fn new(is_muted: bool) -> Self {
        Self { is_muted }
    }
    
    pub fn muted() -> Self {
        Self { is_muted: true }
    }
    
    pub fn unmuted() -> Self {
        Self { is_muted: false }
    }
}

/// Represents the application settings
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub telemetry_enabled: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: false,
        }
    }
    
    pub fn with_telemetry(telemetry_enabled: bool) -> Self {
        Self {
            telemetry_enabled,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
