use std::env;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Can not read project file")]
    Io(#[from] io::Error),
    #[error("Can not get config path")]
    Path,
    #[error("Can not expand path")]
    Expand(#[from] shellexpand::LookupError<env::VarError>),
    #[error("{0}")]
    Message(String),
}
