#![ doc = include_str!("../README.md")]
#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::actions;
use app::actions::NewProjectFrom;
use app::cli::{Cli, Commands};
use clap::Parser;
use error::AppErrorForDisplay;

fn main() -> Result<(), AppErrorForDisplay> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Run { project } => actions::run_project(&project),
        Commands::Stop { project } => actions::stop(&project),
        Commands::Debug { project } => actions::debug_project(&project),
        Commands::Edit { project } => actions::edit_project(&project),
        Commands::Delete { project } => actions::delete_project(&project),
        Commands::New { project, blank } => {
            let new_project = match blank {
                true => NewProjectFrom::Blank { name: &project },
                false => NewProjectFrom::DefaultTemplate { name: &project },
            };
            actions::new_project(&new_project)
        }
        Commands::List => actions::list_projects(),
        Commands::Copy { existing, new } => actions::copy_project(&existing, &new),
        Commands::Doctor => actions::check_config(),
    }
    .map_err(|e| e.into())
}
