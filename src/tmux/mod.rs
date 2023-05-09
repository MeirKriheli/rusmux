//! Tmux operations.
mod commands;
mod error;
mod project;
mod version;

pub use error::TmuxError;
pub use project::TmuxProject;
pub use project::TMUX_BIN;
pub use version::TmuxVersion;
