//! Handles the command requested by the CLI.
use super::config;
use crate::{
    error::AppError,
    project_config::ProjectConfig,
    tmux::{self, TmuxProject},
};
use colored::{ColoredString, Colorize};
use dialoguer::Confirm;
use glob::glob;
use std::{env, fs::copy, process::Command};
use std::{fs::remove_file, io::prelude::*};
use std::{fs::File, path::Path};
use which::which;

/// The default template used when create a new project.
macro_rules! default_template {
    () => {
        "project_name: {}
# project_root: ~/src/project_path
# on_project_start:
#   - sudo systemctl start postgresql
# pre_window:
#   - workon dummy
# windows:
#   - editor: vim
#   - shells:
#       layout: main-vertical
#       panes:
#         - #
#         - grunt serve
"
    };
}

/// List the projects in the configuration directory.
pub(crate) fn list_projects() -> Result<(), AppError> {
    let pattern = config::get_path("*.yml")?;

    for project in glob(&pattern.to_string_lossy()).expect("Failed to glob config dir") {
        match project {
            Ok(path) => println!("{}", &path.file_stem().unwrap().to_string_lossy()),
            Err(e) => println!("{e:?}"),
        }
    }

    Ok(())
}

/// Parses the project file and prints the shell commands for session creation.
pub(crate) fn debug_project(project_name: &str) -> Result<(), AppError> {
    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    println!("{tmux}");
    Ok(())
}

/// Parses the project file, runs the commands to create the tmux session.
pub fn run_project(project_name: &str) -> Result<(), AppError> {
    println!("Starting project {project_name}");

    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    Ok(tmux.run()?)
}

#[doc(hidden)]
/// Helper mapping a [`bool`] value to `"Yes"` or `"No"`.
fn bool_to_yesno(val: bool) -> ColoredString {
    if val {
        "Yes".green()
    } else {
        "No".red()
    }
}

/// Checks environment configuration.
///
/// - `tmux` in `$PATH`.
/// - `$SHELL` is set.
/// - `$EDITOR` are set.
pub(crate) fn check_config() -> Result<(), AppError> {
    let have_tmux = which(tmux::TMUX_BIN).is_ok();
    let have_editor = env::var("EDITOR").is_ok();
    let have_shell = env::var("SHELL").is_ok();

    println!(
        "tmux found?\t{}\n$EDITOR is set?\t{}\n$SHELL is set?\t{}",
        bool_to_yesno(have_tmux),
        bool_to_yesno(have_editor),
        bool_to_yesno(have_shell)
    );

    Ok(())
}

/// Opens an existing project file with `$EDITOR`.
pub(crate) fn edit_project(project_name: &str) -> Result<(), AppError> {
    let project_file_path = config::get_project_path(project_name)?;
    if !Path::new(&project_file_path).exists() {
        return Err(AppError::ProjectFileNotFound(project_file_path));
    }

    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(AppError::EditorNotSet(project_file_path));
    }

    let mut binding = Command::new(editor.unwrap());
    let cmd = binding.arg(project_file_path);

    cmd.status()
        .map_err(|_| AppError::CommandRun(format!("{cmd:?}")))?;
    Ok(())
}

/// Create the new project file from a blank template (only containing) name, or a
/// pre-defined template.
pub enum NewProjectFrom<'a> {
    DefaultTemplate { name: &'a str },
    Blank { name: &'a str },
}

/// Creates a new project file, optinally from [`default_template`], and
/// opens it with `$EDITOR`.
pub(crate) fn new_project(project_from: &NewProjectFrom) -> Result<(), AppError> {
    let project_name = match project_from {
        NewProjectFrom::Blank { name } => name,
        NewProjectFrom::DefaultTemplate { name } => name,
    };

    let project_file_path = config::get_project_path(project_name)?;
    if project_file_path.exists() {
        return Err(AppError::ProjectFileExists(project_file_path));
    }

    // project_name can be a path, so extract the base name from the path as project_name
    let project_name = Path::new(project_name)
        .file_stem()
        .ok_or(AppError::GetProjectNameFromFilePath(
            project_name.to_string(),
        ))?
        .to_string_lossy();
    let content = match project_from {
        NewProjectFrom::Blank { name: _ } => format!("project_name: {project_name}"),
        NewProjectFrom::DefaultTemplate { name: _ } => format!(default_template!(), project_name),
    };

    let mut new_file = File::create(&project_file_path)
        .map_err(|e| AppError::ProjectFileCreate(project_file_path.clone(), e))?;
    new_file
        .write_all(content.as_bytes())
        .map_err(|e| AppError::ProjectFileWrite(project_file_path.clone(), e))?;

    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(AppError::EditorNotSet(project_file_path));
    }

    let mut binding = Command::new(editor.unwrap());
    let cmd = binding.arg(project_file_path);

    cmd.status()
        .map_err(|_| AppError::CommandRun(format!("{cmd:?}")))?;
    Ok(())
}

/// Deletes a project from the configuration directory. Asks for confirmation.
pub(crate) fn delete_project(project_name: &str) -> Result<(), AppError> {
    let project_file_path = config::get_project_path(project_name)?;
    if !project_file_path.exists() {
        return Err(AppError::ProjectFileNotFound(project_file_path));
    }
    let message = format!("Are you sure you want to delete \"{project_name}\"?");
    let confirmation = Confirm::new()
        .with_prompt(message)
        .interact()
        .map_err(AppError::Prompt)?;
    if confirmation {
        remove_file(&project_file_path)
            .map_err(|e| AppError::ProjectFileDelete(project_file_path, e))?;
        println!("Deleted \"{project_name}\"");
    } else {
        println!("Delete aborted");
    }

    Ok(())
}

/// Copies an existing project to a new one, and opens it with `$EDITOR`.
pub(crate) fn copy_project(existing: &str, new: &str) -> Result<(), AppError> {
    let existing_path = config::get_project_path(existing)?;
    if !existing_path.exists() {
        return Err(AppError::ProjectFileNotFound(existing_path));
    }

    let new_path = config::get_project_path(new)?;
    if new_path.exists() {
        return Err(AppError::ProjectFileExists(new_path));
    }

    copy(&existing_path, &new_path)
        .map_err(|e| AppError::ProjectCopy(existing_path, new_path.clone(), e))?;
    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(AppError::EditorNotSet(new_path));
    }

    let mut binding = Command::new(editor.unwrap());
    let cmd = binding.arg(&new_path);

    cmd.status()
        .map_err(|_| AppError::CommandRun(format!("{cmd:?}")))?;
    Ok(())
}

/// Kills the project's session.
pub(crate) fn stop(project_name: &str) -> Result<(), AppError> {
    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    Ok(tmux.stop()?)
}
