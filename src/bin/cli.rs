extern crate grit;

use clap::{value_t, App, Arg, SubCommand};
use grit::commands;
use std::path::PathBuf;

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
        .subcommand(
            SubCommand::with_name("commit")
                .about("Commit some code")
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .help("A helpful message to accompany the commit"),
                ),
        )
        .get_matches();

    if let Some(init) = matches.subcommand_matches("init") {
        match value_t!(init, "directory", PathBuf) {
            Ok(directory) => commands::init(Some(&directory)),
            Err(_) => commands::init(None),
        };
    }

    if let Some(commit) = matches.subcommand_matches("commit") {
        let message = match value_t!(commit, "message", String) {
            Ok(message) => message,
            Err(_) => "".to_string(),
        };
        commands::commit(&message).unwrap();
    }
}
