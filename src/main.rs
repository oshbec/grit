use clap::{App, Arg, SubCommand};
use std::path::PathBuf;

mod init;

fn main() {
    let matches = App::new("grit")
        .version("1.0")
        .about("A rust implementation of git")
        .author("Josh Bechard")
        .subcommand(
            SubCommand::with_name("init")
                .about("Create an empty Git repository or reinitialize an existing one")
                .arg(
                    Arg::with_name("directory")
                        .help("Where the repository lives")
                        .default_value("."),
                ),
        )
        .get_matches();

    if let Some(init) = matches.subcommand_matches("init") {
        if let Some(directory) = init.value_of("directory") {
            init::run(&PathBuf::from(directory));
        }
    }
}
