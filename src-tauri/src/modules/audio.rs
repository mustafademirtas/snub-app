use crate::modules::errors::MicrophoneError;
use crate::modules::types::{MicrophoneState, DEFAULT_UNMUTED_VOLUME, MUTED_VOLUME};

/// macOS-specific audio control functionality
#[cfg(target_os = "macos")]
pub mod macos_audio {
    use super::*;
    use std::process::Command;

    /// Executes an AppleScript command and returns the output
    fn execute_osascript(script: &str) -> Result<String, MicrophoneError> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| MicrophoneError::CommandFailed(format!("Failed to execute osascript: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(MicrophoneError::CommandFailed(format!("osascript failed: {}", error_msg)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Gets the current microphone state from the system
    pub fn get_microphone_state() -> Result<MicrophoneState, MicrophoneError> {
        let volume_str = execute_osascript("input volume of (get volume settings)")?;
        
        let volume: f32 = volume_str.parse()
            .map_err(|_| MicrophoneError::ParseError(format!("Failed to parse volume level: '{}'", volume_str)))?;

        // Consider volume 0 as muted
        Ok(MicrophoneState::new(volume == 0.0))
    }

    /// Sets the microphone mute state
    pub fn set_microphone_mute(mute: bool) -> Result<MicrophoneState, MicrophoneError> {
        let volume_level = if mute { MUTED_VOLUME } else { DEFAULT_UNMUTED_VOLUME };
        let script = format!("set volume input volume {}", volume_level);
        
        execute_osascript(&script)?;
        Ok(MicrophoneState::new(mute))
    }
}

/// Fallback implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub mod macos_audio {
    use super::*;

    pub fn get_microphone_state() -> Result<MicrophoneState, MicrophoneError> {
        Err(MicrophoneError::SystemError("Microphone control is only supported on macOS".to_string()))
    }

    pub fn set_microphone_mute(_mute: bool) -> Result<MicrophoneState, MicrophoneError> {
        Err(MicrophoneError::SystemError("Microphone control is only supported on macOS".to_string()))
    }
}
