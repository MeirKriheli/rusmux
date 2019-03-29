#[macro_use]
extern crate clap;

use clap::{app_from_crate, crate_name, crate_authors, Arg, SubCommand, AppSettings};
use app_dirs::{AppInfo, AppDataType, app_root};

const APP_INFO: AppInfo = AppInfo{
    name: crate_name!(),
    author: crate_authors!()
};


fn list_projects() {
    let config_dir = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    println!("Available projects at {:?}", config_dir);
}

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
            Some("list") => list_projects(),
            None => println!("Please specify a command"),
            _ => println!("TODO run the project"),
        }
    }
}
