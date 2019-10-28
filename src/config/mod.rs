use std::{collections::HashMap, env, path::PathBuf};

pub struct Config {
    pub user: User,
    pub author: User,
    pub committer: User,
}

pub struct User {
    pub name: String,
    pub email: String,
    pub date: String,
}

// Try to get a config value, but default to an empty string
fn extract(config_map: &HashMap<String, String>, key: &str) -> String {
    match config_map.get(key) {
        Some(value) => value.to_owned(),
        None => String::from(""),
    }
}

impl Config {
    // Builds proper config struct from a raw config hashmap
    pub fn build() -> Config {
        let raw_config = merge_configs();
        Config {
            user: User {
                name: extract(&raw_config, "user.name"),
                email: extract(&raw_config, "user.email"),
                date: extract(&raw_config, "user.date"),
            },
            author: User {
                name: extract(&raw_config, "author.name"),
                email: extract(&raw_config, "author.email"),
                date: extract(&raw_config, "author.date"),
            },
            committer: User {
                name: extract(&raw_config, "committer.name"),
                email: extract(&raw_config, "committer.email"),
                date: extract(&raw_config, "committer.date"),
            },
        }
    }
}

// Finds configs wherever they may be (repo, user, system files; env)
// And gives a map with config values back
fn merge_configs() -> HashMap<String, String> {
    // Locate config files
    let repository_config_path = PathBuf::from(".git/config");
    let user_config_path = PathBuf::from("~/.gitconfig");
    let system_config_path = PathBuf::from("/etc/gitconfig");

    // Ordered by least importance, so we can overwrite as we go
    let configurations = vec![
        parse_file_config(&system_config_path),
        parse_file_config(&user_config_path),
        parse_file_config(&repository_config_path),
        parse_env_config(),
    ];

    let mut merged_config: HashMap<String, String> = HashMap::new();
    for configuration in configurations {
        for key in configuration.keys() {
            merged_config.insert(key.to_owned(), configuration[key].to_owned());
        }
    }

    merged_config
}

// Finds stuff like "GIT_AUTHOR_NAME" in env and gives a map with stuff like "author.name"
fn parse_env_config() -> HashMap<String, String> {
    let mut parsed = HashMap::new();
    for (key, value) in env::vars() {
        if !key.starts_with("GIT") {
            continue;
        }
        let mut words: Vec<String> = key
            .split('_')
            .map(|word| word.to_string().to_lowercase())
            .collect();
        words.remove(0);
        let key = words.join(".");

        parsed.insert(key, value);
    }
    parsed
}

// Parse a git config file at a given path
fn parse_file_config(_path: &PathBuf) -> HashMap<String, String> {
    HashMap::new()
}
