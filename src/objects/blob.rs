use crate::objects::{Kind, Object};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Blob {
    pub content: Vec<u8>,
    pub source: PathBuf,
}

impl Blob {
    pub fn from_file<P: Into<PathBuf>>(source: P) -> Blob {
        let source = source.into();
        let content = fs::read_to_string(&source)
            .expect("Couldn't read the file")
            .as_bytes()
            .to_owned();
        Blob { content, source }
    }

    pub fn source(&self) -> &PathBuf {
        &self.source
    }
}

impl Object for Blob {
    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn kind(&self) -> &Kind {
        &Kind::Blob
    }
}
