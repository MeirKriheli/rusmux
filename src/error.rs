//! Top level error module.
use crate::project_config::ProjectParseError;
use crate::tmux::TmuxError;

use std::fmt::{Debug, Display};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Top level error.
#[derive(Error, Debug)]
pub enum AppError {
    /// Problem getting the configuration directory.
    #[error("Can not get config path in the user's home directory")]
    ConfigPath,
    /// Could not show the confirmation prompt.
    #[error("Can not run prompt: {0}")]
    Prompt(dialoguer::Error),
    /// Error during `yaml` parsing.
    #[error("Could not parse yaml from {0}: {1}")]
    YamlParse(PathBuf, String),
    /// Error mapping the parsed yaml to
    /// [ProjectConfig](crate::project_config::project::ProjectConfig).
    #[error("{0}")]
    ProjectParse(#[from] ProjectParseError),
    /// Error running `tmux` operation.
    #[error(transparent)]
    TmuxOperation(#[from] TmuxError),
    /// Problem copying project file from `src` to `dest`
    #[error("Cannoy copy {0} to {1}: {2}")]
    ProjectCopy(PathBuf, PathBuf, io::Error),
    /// Problem creating the configuration directory.
    #[error("Could not create config dir {0}: {1}")]
    ProjectCreateConfigDir(PathBuf, io::Error),
    /// Error during file creation.
    #[error("Could not create project file {0}: {1}")]
    ProjectFileCreate(PathBuf, io::Error),
    /// Error during file deletion.
    #[error("Can not delete Project file {0}: {1}")]
    ProjectFileDelete(PathBuf, io::Error),
    /// The project file already exists.
    #[error("Project file {0} already exists")]
    ProjectFileExists(PathBuf),
    /// Can not find the project file.
    #[error("Project file {0} not found")]
    ProjectFileNotFound(PathBuf),
    /// Error reading file content.
    #[error("Could not read content from project file {0}: {1}")]
    ProjectFileRead(PathBuf, io::Error),
    /// Error writing file content.
    #[error("Could not write content to project file {0}: {1}")]
    ProjectFileWrite(PathBuf, io::Error),
    /// `$EDITOR` environment var is not set.
    #[error("$EDITOR is not set, the file path to edit is {0}")]
    EditorNotSet(PathBuf),
    /// Error running a command
    #[error("Could not run command {0}")]
    CommandRun(String),
    /// Error expanding a directory/file path.
    #[error("Can not expand path")]
    Expand(#[from] shellexpand::LookupError<std::env::VarError>),
    /// Error getting project name from file path
    #[error("Can not get project name from file path {0}")]
    GetProjectNameFromFilePath(String),
    #[error("Failed to glob config dir {0}")]
    ProjectGlob(#[from] glob::PatternError),
}

/// Used for displaying the error on exit.
///
/// By default, existing with an error from `main()` displays the
/// debug information of the error, which is not human friendly.
///
/// To mitigate, this wraps [AppError], and implements [Debug] as
/// [Display].
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
