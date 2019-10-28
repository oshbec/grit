use crate::objects::{Kind, Object};
use chrono::Local;

use crate::config::Config;

#[derive(Debug)]
pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new(tree_id: String, message: &str) -> Commit {
        let config = Config::build();
        let author_name = config.author.name;
        let author_email = config.author.email;
        let author_date = match config.author.date.len() {
            0 => Local::now().format("%s %z").to_string(),
            _ => config.author.date,
        };
        let author = format!("{} <{}> {}", author_name, author_email, author_date);
        let content = format!(
            "tree {}\nauthor {}\ncommitter {}\n\n{}\n",
            tree_id, author, author, message
        )
        .as_bytes()
        .to_owned();
        Commit { content }
    }
}

impl Object for Commit {
    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn kind(&self) -> &Kind {
        &Kind::Commit
    }
}
