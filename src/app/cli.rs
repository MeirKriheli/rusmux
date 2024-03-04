//! CLI arguments parser.
use clap::{command, Arg, ArgAction, Command};
pub(crate) fn arg_is_path() -> Arg {
    Arg::new("path")
        .long("path")
        .short('p')
        .action(ArgAction::SetTrue)
        .help("Load from path instead of project")
}

pub(crate) fn arg_project() -> Arg {
    Arg::new("project").help("Project name").required(true)
}

/// Returns [`clap::Command`] for the CLI application.
pub(crate) fn get_cli_command_parser() -> Command {
    command!()
        .arg_required_else_help(true)
        .subcommand_negates_reqs(true)
        .arg(arg_project())
        .arg(arg_is_path())
        .subcommand(Command::new("list").about("List all projects"))
        .subcommand(
            Command::new("debug")
                .about("Output shell commands for a project")
                .arg(arg_is_path())
                .arg(arg_project()),
        )
        .subcommand(
            Command::new("run")
                .about("Run the project commands")
                .arg(arg_is_path())
                .arg(arg_project()),
        )
        .subcommand(
            Command::new("copy")
                .about("Copy an existing project to a new one and edit it")
                .arg(
                    Arg::new("existing")
                        .help("Existing project name")
                        .required(true),
                )
                .arg(Arg::new("new").help("New project name").required(true)),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit an existing project")
                .arg(arg_is_path())
                .arg(arg_project()),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an existing project")
                .arg(arg_project()),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(arg_project())
                .arg(
                    Arg::new("blank")
                        .long("blank")
                        .action(ArgAction::SetTrue)
                        .help("Don't use a template for the file"),
                ),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop the project's session")
                .arg(arg_is_path())
                .arg(arg_project()),
        )
        .subcommand(Command::new("doctor").about("Check your configuration"))
}
