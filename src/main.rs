#![ doc = include_str!("../README.md")]
#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::actions::NewProjectFrom;
use app::{actions, cli};
use error::AppErrorForDisplay;

fn main() -> Result<(), AppErrorForDisplay> {
    let matches = cli::get_cli_command_parser().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => actions::list_projects(),
        Some(("doctor", _)) => actions::check_config(),
        Some(("debug", debug_matches)) => {
            actions::debug_project(debug_matches.get_one::<String>("project").unwrap())
        }
        Some(("run", run_matches)) => {
            actions::run_project(run_matches.get_one::<String>("project").unwrap())
        }
        Some(("edit", edit_matches)) => {
            actions::edit_project(edit_matches.get_one::<String>("project").unwrap())
        }
        Some(("delete", delete_matches)) => {
            actions::delete_project(delete_matches.get_one::<String>("project").unwrap())
        }
        Some(("new", new_matches)) => {
            let name = new_matches.get_one::<String>("project").unwrap();
            let is_blank = *new_matches.get_one::<bool>("blank").unwrap();
            let new_project = match is_blank {
                true => NewProjectFrom::Blank { name },
                false => NewProjectFrom::DefaultTemplate { name },
            };
            actions::new_project(&new_project)
        }
        Some(("copy", copy_matches)) => actions::copy_project(
            copy_matches.get_one::<String>("existing").unwrap(),
            copy_matches.get_one::<String>("new").unwrap(),
        ),
        Some(("stop", stop_matches)) => {
            actions::stop(stop_matches.get_one::<String>("project").unwrap())
        }
        _ => unreachable!(),
    }
    .map_err(|e| e.into())
}
