use crate::modules::errors::MicrophoneError;
use std::process::Command;

/// macOS-specific sound playback functionality
#[cfg(target_os = "macos")]
pub mod macos_sound {
    use super::*;

    /// Plays a system sound using afplay
    fn play_system_sound(sound_name: &str) -> Result<(), MicrophoneError> {
        let sound_path = format!("/System/Library/Sounds/{}.aiff", sound_name);
        
        let output = Command::new("afplay")
            .arg(&sound_path)
            .output()
            .map_err(|e| MicrophoneError::CommandFailed(format!("Failed to play sound: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(MicrophoneError::CommandFailed(format!("afplay failed: {}", error_msg)));
        }

        Ok(())
    }

    /// Plays a mute sound (lower pitch)
    pub fn play_mute_sound() -> Result<(), MicrophoneError> {
        // Try system sounds in order of preference
        let sounds = ["Tink", "Pop", "Morse"];
        
        for sound in &sounds {
            if let Ok(_) = play_system_sound(sound) {
                return Ok(());
            }
        }
        
        // If no system sound works, we'll just succeed silently
        Ok(())
    }

    /// Plays an unmute sound (higher pitch)
    pub fn play_unmute_sound() -> Result<(), MicrophoneError> {
        // Try system sounds in order of preference
        let sounds = ["Tink", "Pop", "Morse"];
        
        for sound in &sounds {
            if let Ok(_) = play_system_sound(sound) {
                return Ok(());
            }
        }
        
        // If no system sound works, we'll just succeed silently
        Ok(())
    }
}

/// Fallback implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub mod macos_sound {
    use super::*;

    pub fn play_mute_sound() -> Result<(), MicrophoneError> {
        // No-op for non-macOS platforms
        Ok(())
    }

    pub fn play_unmute_sound() -> Result<(), MicrophoneError> {
        // No-op for non-macOS platforms
        Ok(())
    }
}
