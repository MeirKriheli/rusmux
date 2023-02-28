use super::config;
use crate::{
    error::AppError,
    project_config::ProjectConfig,
    tmux::{self, TmuxProject},
};
use glob::glob;
use std::{env, process::Command};
use std::path::Path;
use which::which;

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

    Command::new(editor.unwrap()).arg(project_file_path).status()?;
    Ok(())
}
