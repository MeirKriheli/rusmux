#[macro_use]
extern crate clap;

mod config;
mod error;
mod project;
mod stringorvec;
mod tmux;
mod window;

use clap::{command, Arg, Command};
use error::AppError;
use glob::glob;
use project::ProjectConfig;
use std::convert::TryFrom;
use std::path::Path;
use tmux::TmuxProject;

/// List the project files in the configuration directory
fn list_projects() -> Result<(), AppError> {
    let pattern = config::get_path("*.yml")?;

    for project in glob(&pattern).expect("Failed to glob config dir") {
        match project {
            Ok(path) => println!(
                "{}",
                Path::new(&path).file_stem().unwrap().to_str().unwrap()
            ),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}

// Parses the project file, creates the tmux session
fn run_project(project_name: &str) -> Result<(), AppError> {
    println!("Starting project {}", project_name);

    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    tmux.run()
}

// Parses the project file, prints shell commands
fn debug_project(project_name: &str) -> Result<(), AppError> {
    let entries = config::get_project_yaml(project_name)?;
    let project = ProjectConfig::try_from(entries)?;
    let tmux = TmuxProject::new(&project)?;
    println!("{}", tmux);
    Ok(())
}

fn main() -> Result<(), AppError> {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_negates_reqs(true)
        .arg(Arg::new("project").help("Project name").required(true))
        .subcommand(Command::new("list").about("List all projects"))
        .subcommand(
            Command::new("debug")
                .about("Output shell commands for a project")
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .subcommand(
            Command::new("run")
                .about("Run the project commands")
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .get_matches();

    if let Some(project_name) = matches.get_one::<String>("project") {
        return run_project(project_name);
    }

    match matches.subcommand() {
        Some(("list", _)) => list_projects(),
        Some(("debug", debug_matches)) => {
            debug_project(debug_matches.get_one::<String>("project").unwrap())
        }
        Some(("run", run_matches)) => {
            run_project(run_matches.get_one::<String>("project").unwrap())
        }
        _ => unreachable!(),
    }
}
