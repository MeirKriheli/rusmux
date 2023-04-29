//! Tmux operations errors.
use thiserror::Error;

/// Indicates a problem running tmux commands and operations error
#[derive(Error, Debug)]
pub enum TmuxError {
    /// Error running tmux command
    #[error("Error running command")]
    CommandError(#[from] std::io::Error),
    /// Generic string error message, (e.g.: _problem starting a session_).
    #[error("{0}")]
    Message(String),
    /// Error expanding a directory/file path.
    #[error("Can not expand path")]
    Expand(#[from] shellexpand::LookupError<std::env::VarError>),
}
