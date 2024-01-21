//! CLI arguments parser.
use clap::{command, Arg, ArgAction, Command};

/// Returns [`clap::Command`] for the CLI application.
pub(crate) fn get_cli_command_parser() -> Command {
    command!()
        .arg_required_else_help(true)
        .subcommand_negates_reqs(true)
        .arg(Arg::new("project").help("Project name").required(true))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .action(ArgAction::SetTrue)
                .help("Load from path instead of project"),
        )
        .subcommand(Command::new("list").about("List all projects"))
        .subcommand(
            Command::new("debug")
                .about("Output shell commands for a project")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .action(ArgAction::SetTrue)
                        .help("Load from path instead of project"),
                )
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .subcommand(
            Command::new("run")
                .about("Run the project commands")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .action(ArgAction::SetTrue)
                        .help("Load from path instead of project"),
                )
                .arg(Arg::new("project").help("Project name").required(true)),
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
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .action(ArgAction::SetTrue)
                        .help("Load from path instead of project"),
                )
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an existing project")
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(Arg::new("project").help("Project name").required(true))
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
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .action(ArgAction::SetTrue)
                        .help("Load from path instead of project"),
                )
                .arg(Arg::new("project").help("Project name").required(true)),
        )
        .subcommand(Command::new("doctor").about("Check your configuration"))
}
