use crate::objects::{Kind, Object, Tree};
use std::env;
use time;

#[derive(Debug)]
pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new(tree: Tree, message: Option<String>) -> Commit {
        let author_name = env::var("GIT_AUTHOR_NAME").unwrap();
        let author_email = env::var("GIT_AUTHOR_EMAIL").unwrap();
        let timestamp = format!("{}", time::now().strftime("%s %z").unwrap());
        let author = format!("{} <{}> {}", author_name, author_email, timestamp);
        let content = format!(
            "tree {}\nauthor {}\ncommitter {}\n\n{}\n",
            tree.id(),
            author,
            author,
            message.unwrap()
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
