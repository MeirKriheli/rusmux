use clap::{command, Arg, Command};

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
        .subcommand(Command::new("doctor").about("Check your configuration"))
}
