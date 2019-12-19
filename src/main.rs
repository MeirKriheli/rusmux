#[macro_use]
extern crate clap;

mod command;
mod config;
mod project;

use clap::{app_from_crate, crate_authors, crate_name, AppSettings, Arg, SubCommand};
use glob::glob;
use std::path::Path;

use project::Project;

// List the project files in the configuration directory
fn list_projects() {
    let pattern = config::get_path(&"*.yml");

    for project in glob(&pattern).expect("Failed to glob config dir") {
        match project {
            Ok(path) => println!(
                "{}",
                Path::new(&path).file_stem().unwrap().to_str().unwrap()
            ),
            Err(e) => println!("{:?}", e),
        }
    }
}

// Parses the project file, creates the tmux session
fn run_project(project_name: &str) {
    println!("Starting project {}", project_name);

    let entries = config::get_project_yaml(&project_name);
    let project = Project::from(entries);
    println!("{:#?}", project);
}

// Parses the project file, prints shell commands
fn debug_project(project_name: &str) {
    let entries = config::get_project_yaml(&project_name);
    let project = Project::from(entries);
    println!("{:#?}", project);
}

fn main() {
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
        _ => println!("{}\nRerun with --help for more info", matches.usage()),
    }
}
