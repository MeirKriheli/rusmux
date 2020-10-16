#[macro_use]
extern crate clap;

mod commands;
mod config;
mod stringorvec;
mod error;
mod project;
mod window;

use clap::{app_from_crate, crate_authors, crate_name, AppSettings, Arg, SubCommand};
use error::AppError;
use glob::glob;
use project::Project;
use std::convert::TryFrom;
use std::path::Path;

/// List the project files in the configuration directory
fn list_projects() -> Result<(), AppError> {
    let pattern = config::get_path(&"*.yml")?;

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

    let entries = config::get_project_yaml(&project_name)?;
    let project = Project::try_from(entries)?;
    println!("{:#?}", project);
    Ok(())
}

// Parses the project file, prints shell commands
fn debug_project(project_name: &str) -> Result<(), AppError> {
    let entries = config::get_project_yaml(&project_name)?;
    let project = Project::try_from(entries)?;
    project.debug();
    Ok(())
}

fn main() -> Result<(), AppError> {
    let matches = app_from_crate!()
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("list").about("List all projects"))
        .subcommand(
            SubCommand::with_name("debug")
                .about("Output shell commands for a project")
                .arg(
                    Arg::with_name("project")
                        .help("Project name")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", Some(_)) => list_projects(),
        ("debug", Some(debug_matches)) => debug_project(debug_matches.value_of("project").unwrap()),
        (project_name, Some(_)) => run_project(project_name),
        _ => Err(AppError::Message(format!(
            "{}\nRerun with --help for more info",
            matches.usage()
        ))),
    }
}
