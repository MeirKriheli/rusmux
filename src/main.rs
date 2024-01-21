#![ doc = include_str!("../README.md")]
#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::{actions, cli};
use error::AppErrorForDisplay;

fn main() -> Result<(), AppErrorForDisplay> {
    let matches = cli::get_cli_command_parser().get_matches();

    if let Some(project_name) = matches.get_one::<String>("project") {
        let is_path = matches.get_one::<bool>("path").unwrap();
        return actions::run_project(project_name, *is_path).map_err(|e| e.into());
    }

    match matches.subcommand() {
        Some(("list", _)) => actions::list_projects(),
        Some(("doctor", _)) => actions::check_config(),
        Some(("debug", debug_matches)) => {
            let project = debug_matches.get_one::<String>("project").unwrap();
            let is_path = debug_matches.get_one::<bool>("path").unwrap();
            actions::debug_project(project, *is_path)
        }
        Some(("run", run_matches)) => {
            let project = run_matches.get_one::<String>("project").unwrap();
            let is_path = run_matches.get_one::<bool>("path").unwrap();
            actions::run_project(project, *is_path)
        }
        Some(("edit", edit_matches)) => {
            actions::edit_project(edit_matches.get_one::<String>("project").unwrap())
        }
        Some(("delete", delete_matches)) => {
            actions::delete_project(delete_matches.get_one::<String>("project").unwrap())
        }
        Some(("new", new_matches)) => actions::new_project(
            new_matches.get_one::<String>("project").unwrap(),
            *new_matches.get_one::<bool>("blank").unwrap(),
        ),
        Some(("copy", copy_matches)) => actions::copy_project(
            copy_matches.get_one::<String>("existing").unwrap(),
            copy_matches.get_one::<String>("new").unwrap(),
        ),
        Some(("stop", stop_matches)) => {
            let project = stop_matches.get_one::<String>("project").unwrap();
            let is_path = stop_matches.get_one::<bool>("path").unwrap();
            actions::stop(project, *is_path)
        }
        _ => unreachable!(),
    }
    .map_err(|e| e.into())
}
