use clap::{value_t, App, Arg, SubCommand};
use std::{env, path::PathBuf};

mod compression;
mod ignore;
mod objects;
mod repository;

use objects::Object;
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
        .subcommand(SubCommand::with_name("commit").about("Commit some code"))
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

    if let Some(_init) = matches.subcommand_matches("commit") {
        commit();
    }
}

fn commit() {
    let current_dir = env::current_dir().unwrap();
    let repository = Repository::at(Some(current_dir));
    let files_to_commit = repository.list_files();
    for file in &files_to_commit {
        let object = Object::from_file(file);
        object
            .write()
            .expect("Couldn't write entry to git database");
    }
    println!("Going to commit these files: {:?}", files_to_commit);
}
