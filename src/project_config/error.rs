//! Project config parsing error.
use std::{error::Error, fmt::Display};

/// The project config parsing error, wrapping the de-serialization error.
#[derive(Debug)]
pub struct ProjectParseError(pub String);

impl Error for ProjectParseError {}

impl Display for ProjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse yaml: {}", self.0)
    }
}
