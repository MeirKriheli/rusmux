use clap::{command, Arg, ArgAction, Command};

/// Returns the `clap::Commnad` for the application
pub(crate) fn get_cli_command_parser() -> Command {
    command!()
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
        .subcommand(
            Command::new("edit")
                .about("Edit an existing project")
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
        .subcommand(Command::new("doctor").about("Check your configuration"))
}
