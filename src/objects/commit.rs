use crate::objects::{Kind, Object};
use chrono::Local;
use std::env;

#[derive(Debug)]
pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new(tree_id: String, message: &str) -> Commit {
        let author_name = env::var("GIT_AUTHOR_NAME").expect("Couldn't determine author name");
        let author_email = env::var("GIT_AUTHOR_EMAIL").expect("Couldn't determine author email");
        let author_date = match env::var("GIT_AUTHOR_DATE") {
            Ok(date) => date,
            Err(_) => Local::now().format("%s %z").to_string(),
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
