#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::{actions, cli};
use error::AppError;

fn main() -> Result<(), AppError> {
    let matches = cli::get_cli_command_parser().get_matches();

    if let Some(project_name) = matches.get_one::<String>("project") {
        return actions::run_project(project_name);
    }

    match matches.subcommand() {
        Some(("list", _)) => actions::list_projects(),
        Some(("doctor", _)) => actions::check_config(),
        Some(("debug", debug_matches)) => {
            actions::debug_project(debug_matches.get_one::<String>("project").unwrap())
        }
        Some(("run", run_matches)) => {
            actions::run_project(run_matches.get_one::<String>("project").unwrap())
        }
        _ => unreachable!(),
    }
}
