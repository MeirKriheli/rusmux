use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmuxError {
    #[error("Error running command")]
    CommandError(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
    #[error("Can not expand path")]
    Expand(#[from] shellexpand::LookupError<std::env::VarError>),
}
