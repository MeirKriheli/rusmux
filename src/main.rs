#[macro_use]
extern crate clap;

use app_dirs::{app_root, AppDataType, AppInfo};
use clap::{app_from_crate, crate_authors, crate_name, AppSettings, Arg, SubCommand};
use glob::glob;
use std::path::Path;

const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: crate_authors!(),
};

fn list_projects() {
    let config_dir = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();

    let pattern = Path::new(&config_dir)
        .join("*.yml")
        .into_os_string()
        .into_string()
        .unwrap();

    for project in glob(&pattern).expect("Failed to glob config dir") {
        match project {
           Ok(path) =>  println!("{}", Path::new(&path).file_stem().unwrap().to_str().unwrap()),
           Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("command")
                .help("Subcommand or project name")
                .required(true),
        )
        .subcommand(SubCommand::with_name("list").about("List all projects"))
        .get_matches();

    if let Some(project_name) = matches.value_of("command") {
        println!("{} project", project_name);
    } else {
        match matches.subcommand_name() {
            Some("list") => list_projects(),
            None => println!("Please specify a command"),
            _ => println!("TODO run the project"),
        }
    }
}
