use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ProjectParseError(pub String);

impl Error for ProjectParseError {}

impl Display for ProjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot parse yaml: {}", self.0)
    }
}
