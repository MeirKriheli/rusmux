//! Handles mapping the project's configuration from `.yml' files to
//! [ProjectConfig] with [serde].
//!
//! Uses 2 custom visitors:
//!
//! * `WindowVisitor` - Handles cases where a window can be a string (single
//!   pane command), or a vector of several panes.
//! * `OptionalVecOrStringVisitor` - Handles cases where the value is optional
//!   and can contain a string or a vector of strings.
//!
//! A project must include the name, optional events (like `start`, `stop`),
//! and optional windows, each with optional panes.

pub mod error;
pub mod project;
mod stringorvec;
pub mod window;

pub use error::ProjectParseError;
pub use project::ProjectConfig;
pub use window::Window;
