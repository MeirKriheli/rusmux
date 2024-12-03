//! CLI arguments parser.
use clap::{command, Arg, ArgAction, Command};

/// Returns [`clap::Command`] for the CLI application.
pub(crate) fn get_cli_command_parser() -> Command {
    command!()
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("List all projects"))
        .subcommand(
            Command::new("debug")
                .about("Output shell commands for a project")
                .arg(
                    Arg::new("project")
                        .help("Project name or filesystem path")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("run")
                .alias("start")
                .about("Run the project commands")
                .arg(
                    Arg::new("project")
                        .help("Project name or filesystem path")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("copy")
                .about("Copy an existing project to a new one and edit it")
                .arg(
                    Arg::new("existing")
                        .help("Existing Project name or filesystem path")
                        .required(true),
                )
                .arg(
                    Arg::new("new")
                        .help("New Project name or filesystem path")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("edit").about("Edit an existing project").arg(
                Arg::new("project")
                    .help("Project name or filesystem path")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an existing project")
                .arg(
                    Arg::new("project")
                        .help("Project name or filesystem path")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(
                    Arg::new("project")
                        .help("Project name or filesystem path")
                        .required(true),
                )
                .arg(
                    Arg::new("blank")
                        .long("blank")
                        .action(ArgAction::SetTrue)
                        .help("Don't use a template for the file"),
                ),
        )
        .subcommand(
            Command::new("stop")
                .alias("kill")
                .about("Stop the project's session")
                .arg(
                    Arg::new("project")
                        .help("Project name or filesystem path")
                        .required(true),
                ),
        )
        .subcommand(Command::new("doctor").about("Check your configuration"))
}
