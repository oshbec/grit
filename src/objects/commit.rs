use crate::objects::{Kind, Object};
use std::env;

#[derive(Debug)]
pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new(tree_id: String, message: &str) -> Commit {
        let author_name = env::var("GIT_AUTHOR_NAME").unwrap();
        let author_email = env::var("GIT_AUTHOR_EMAIL").unwrap();
        let author_date = env::var("GIT_AUTHOR_DATE").unwrap();
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
