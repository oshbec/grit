use clap::{value_t, App, Arg, SubCommand};
use std::path::PathBuf;

mod repository;

use repository::Repository;

fn main() {
    let matches = App::new("grit")
        .version("1.0")
        .about("A rust implementation of git")
        .author("Josh Bechard")
        .subcommand(
            SubCommand::with_name("init")
                .about("Create an empty Git repository or reinitialize an existing one")
                .arg(Arg::with_name("directory").help("Where the repository lives")),
        )
        .get_matches();

    if let Some(init) = matches.subcommand_matches("init") {
        let directory = match value_t!(init, "directory", PathBuf) {
            Ok(directory) => Some(directory),
            Err(_) => None,
        };
        match Repository::at(directory).init() {
            Ok(_) => return,
            Err(e) => println!("Failed to initialize repository: {}", e),
        };
    }
}
