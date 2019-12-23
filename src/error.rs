use app_dirs::AppDirsError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Can not read project file")]
    Io(#[from] io::Error),
    #[error("Can not read config dir")]
    AppDir(#[from] AppDirsError),
    #[error("Can not get config path")]
    Path,
    #[error("{0}")]
    Message(String),
}
