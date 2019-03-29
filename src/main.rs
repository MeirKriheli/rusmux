#[macro_use]
extern crate clap;

use clap::{app_from_crate, Arg};

fn main() {
    let _ = app_from_crate!()
        .arg(Arg::with_name("COMMAND")
                .help("Command or project name to execute")
                .required(true)
             )
        .get_matches();
}
