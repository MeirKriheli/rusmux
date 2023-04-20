use crate::project_config::ProjectParseError;
use crate::tmux::TmuxError;

use std::fmt::{Debug, Display};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Can not get config path in the user's home directory")]
    Path,
    #[error("Can not run prompt: {0}")]
    Prompt(io::Error),
    #[error("{0}")]
    ProjectParseError(#[from] ProjectParseError),
    #[error("Could not parse yaml from {0}: {1}")]
    YamlParseError(PathBuf, String),
    #[error(transparent)]
    TmuxError(#[from] TmuxError),
    #[error("Cannoy copy {0} to {1}: {2}")]
    ProjectCopy(PathBuf, PathBuf, io::Error),
    #[error("Could not create config dir {0}: {1}")]
    ProjectCreateConfigDir(PathBuf, io::Error),
    #[error("Could not create project file {0}: {1}")]
    ProjectFileCreate(PathBuf, io::Error),
    #[error("Can not delete Project file {0}: {1}")]
    ProjectFileDelete(PathBuf, io::Error),
    #[error("Project file {0} already exists")]
    ProjectFileExists(PathBuf),
    #[error("Project file {0} not found")]
    ProjectFileNotFound(PathBuf),
    #[error("Could not read content from project file {0}: {1}")]
    ProjectFileReadError(PathBuf, io::Error),
    #[error("Could not write content to project file {0}: {1}")]
    ProjectFileWriteError(PathBuf, io::Error),
    #[error("$EDITOR is not set, the file path to edit is {0}")]
    EditorNotSet(PathBuf),
    #[error("Could not run command {0}")]
    CommandRunError(String),
}

/// Used for displaying the error on exit.
///
/// By default existing with an error from main, it displays the
/// debug version, which is not human friendly.
///
/// To mitigate, it wraps [AppError], and implements [Debug] just
/// like [Display].
#[derive(Error)]
pub(crate) struct AppErrorForDisplay(AppError);

impl Display for AppErrorForDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for AppErrorForDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AppError> for AppErrorForDisplay {
    fn from(value: AppError) -> Self {
        Self(value)
    }
}
