use crate::objects::{Kind, Object};
use std::env;
use time;

#[derive(Debug)]
pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new(tree_id: String, message: &str, date: Option<time::Tm>) -> Commit {
        let right_now = time::now();
        let date = date.unwrap_or(right_now);
        let author_name = env::var("GIT_AUTHOR_NAME").unwrap();
        let author_email = env::var("GIT_AUTHOR_EMAIL").unwrap();
        let timestamp = format!("{}", date.strftime("%s %z").expect("Could not format date"));
        let author = format!("{} <{}> {}", author_name, author_email, timestamp);
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
