#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::actions;
use clap::{command, Arg, Command};
use error::AppError;

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
        .subcommand(Command::new("doctor").about("Check your configuration"))
        .get_matches();

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
