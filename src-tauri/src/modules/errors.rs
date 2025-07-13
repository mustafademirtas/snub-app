use std::error::Error;
use std::fmt;

/// Custom error type for microphone operations
#[derive(Debug)]
pub enum MicrophoneError {
    CommandFailed(String),
    ParseError(String),
    SystemError(String),
}

impl fmt::Display for MicrophoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MicrophoneError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            MicrophoneError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MicrophoneError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl Error for MicrophoneError {}

impl From<MicrophoneError> for String {
    fn from(error: MicrophoneError) -> Self {
        error.to_string()
    }
}

/// Logs errors to stderr with context
pub fn log_error(context: &str, error: impl std::fmt::Display) {
    eprintln!("{}: {}", context, error);
}
