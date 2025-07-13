use serde::{Deserialize, Serialize};

// Constants for better maintainability
pub const DEFAULT_UNMUTED_VOLUME: &str = "50";
pub const MUTED_VOLUME: &str = "0";
pub const TRAY_ID: &str = "main";
pub const MAIN_WINDOW_ID: &str = "main";

// Menu item IDs
pub const MENU_TOGGLE_MIC: &str = "toggle_microphone";
pub const MENU_SHOW_WINDOW: &str = "show_window";
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
