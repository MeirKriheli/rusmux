#[macro_use]
extern crate clap;

mod project;

use app_dirs::{app_root, AppDataType, AppInfo};
use clap::{app_from_crate, crate_authors, crate_name, AppSettings, Arg, SubCommand};
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use yaml_rust::yaml;
use project::Project;

const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: crate_authors!(),
};

// Return config directory, which stores project files
fn get_config_dir() -> PathBuf {
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

// Returns the path of a file/pattern inside the donfig dir
fn get_config_path(pattern: &str) -> String {
    let config_dir = get_config_dir();

    Path::new(&config_dir)
        .join(pattern)
        .into_os_string()
        .into_string()
        .unwrap()
}

// List the project files in the configuration directory
fn list_projects() {
    let pattern = get_config_path("*.yml");

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

    let mut filename = project_name.to_owned();
    filename.push_str(".yml");

    let project_file_path = get_config_path(&filename);
    let mut project_file = File::open(project_file_path).unwrap();
    let mut contents = String::new();
    project_file.read_to_string(&mut contents).unwrap();

    let entries = yaml::YamlLoader::load_from_str(&contents).unwrap();

    for entry in &entries {
        let project = Project::new(entry);
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
