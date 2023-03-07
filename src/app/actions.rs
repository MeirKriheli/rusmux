use super::config;
use crate::{
    error::AppError,
    project_config::ProjectConfig,
    tmux::{self, TmuxProject},
};
use dialoguer::Confirm;
use glob::glob;
use std::{env, process::Command};
use std::{fs::remove_file, io::prelude::*};
use std::{fs::File, path::Path};
use which::which;

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

/// List the project files in the configuration directory
pub(crate) fn list_projects() -> Result<(), AppError> {
    let pattern = config::get_path("*.yml")?;

    for project in glob(&pattern).expect("Failed to glob config dir") {
        match project {
            Ok(path) => println!(
                "{}",
                Path::new(&path).file_stem().unwrap().to_str().unwrap()
            ),
            Err(e) => println!("{e:?}"),
        }
    }

    Ok(())
}

// Parses the project file, prints shell commands
pub(crate) fn debug_project(project_name: &str) -> Result<(), AppError> {
    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    println!("{tmux}");
    Ok(())
}

/// Parses the project file, creates the tmux session
pub fn run_project(project_name: &str) -> Result<(), AppError> {
    println!("Starting project {project_name}");

    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    Ok(tmux.run()?)
}

fn bool_to_yesno(val: bool) -> &'static str {
    if val {
        "Yes"
    } else {
        "No"
    }
}

// Check the configuration
pub(crate) fn check_config() -> Result<(), AppError> {
    let have_tmux = which(tmux::TMUX_BIN).is_ok();
    let have_editor = env::var("EDITOR").is_ok();
    let have_shell = env::var("SHELL").is_ok();

    println!(
        "tmux is installed? {}\n$EDITOR is set? {}\n$SHELL is set? {}",
        bool_to_yesno(have_tmux),
        bool_to_yesno(have_editor),
        bool_to_yesno(have_shell)
    );

    Ok(())
}

pub(crate) fn edit_project(project_name: &str) -> Result<(), AppError> {
    let filename = format!("{}.yml", project_name);
    let project_file_path = config::get_path(&filename)?;
    if !Path::new(&project_file_path).exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
    }

    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(AppError::Message(format!(
            "$EDITOR is not set, the file path to edit is {project_file_path}"
        )));
    }

    Command::new(editor.unwrap())
        .arg(project_file_path)
        .status()?;
    Ok(())
}

pub(crate) fn new_project(project_name: &str, blank: bool) -> Result<(), AppError> {
    let filename = format!("{}.yml", project_name);
    let project_file = config::get_path(&filename)?;
    let project_file_path = Path::new(&project_file);
    if project_file_path.exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists).into());
    }
    let content = if blank {
        format!("project_name: {project_name}")
    } else {
        format!(default_template!(), project_name)
    };

    let mut new_file = File::create(project_file_path)?;
    new_file.write_all(content.as_bytes())?;

    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(AppError::Message(format!(
            "$EDITOR is not set, created the file {project_file}"
        )));
    }

    Command::new(editor.unwrap())
        .arg(project_file_path)
        .status()?;
    Ok(())
}

pub(crate) fn delete_project(project_name: &str) -> Result<(), AppError> {
    let filename = format!("{}.yml", project_name);
    let project_file = config::get_path(&filename)?;
    let project_file_path = Path::new(&project_file);
    if !project_file_path.exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
    }
    let message = format!("Are you sure you want to delete \"{project_name}\"?");
    if Confirm::new().with_prompt(message).interact()? {
        remove_file(project_file_path)?;
        println!("Deleted \"{project_name}\"");
    } else {
        println!("Delete aborted");
    }

    Ok(())
}
