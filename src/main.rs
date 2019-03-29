#[macro_use]
extern crate clap;

use clap::{app_from_crate, Arg, SubCommand, AppSettings};

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("command")
                    .help("Subcommand or project name")
                    .required(true)
                )
        .subcommand(SubCommand::with_name("list")
                       .about("List all projects"))
        .get_matches();

    if let Some(project_name) = matches.value_of("command") {
        println!("{} project", project_name);
    }
    else {
        match matches.subcommand_name() {
            Some("list") => println!("Available projects"),
            None => println!("Please specify a command"),
            _ => println!("TODO run the project"),
        }
    }
}
