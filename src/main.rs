#[macro_use]
extern crate clap;

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

    for entry in entries {
        let project = Project::new(&entry);
        println!("{:#?}", project);
    }
}

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("command")
                .help("Subcommand or project name")
                .required(true),
        )
        .subcommand(SubCommand::with_name("list").about("List all projects"))
        .get_matches();

    if let Some(project_name) = matches.value_of("command") {
        run_project(project_name);
    } else {
        match matches.subcommand_name() {
            Some("list") => list_projects(),
            None => println!("Please specify a command"),
            _ => println!("Should not get here"),
        }
    }
}
