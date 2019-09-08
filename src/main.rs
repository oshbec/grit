use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::PathBuf;

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
            let mut directory = PathBuf::from(directory);
            directory.push(".git");
            fs::create_dir_all(&directory).unwrap();
            directory.push("objects");
            fs::create_dir_all(&directory).unwrap();
            directory.pop();
            directory.push("refs");
            fs::create_dir_all(&directory).unwrap();
        }
    }
}

#[cfg(test)]
mod dummy_tests {

    #[test]
    fn passes() {
        assert!(true);
    }

}
