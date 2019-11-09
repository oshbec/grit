use crate::objects::{blob::Blob, Kind, Object};
use hex;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Tree {
    blobs: Vec<Blob>,
    content: Vec<u8>,
}

#[allow(dead_code)]
impl Tree {
    /// Build a new tree from a collection of Blobs
    pub fn from_blobs(blobs: Vec<Blob>) -> Tree {
        let mut blobs = blobs;

        blobs.sort_by(|a, b| {
            let source_of_a = a.source();
            let source_of_b = b.source();
            source_of_a
                .to_str()
                .unwrap()
                .cmp(source_of_b.to_str().unwrap())
        });

        let content: Vec<u8> = blobs
            .iter()
            .map(|blob| {
                let mut binary_blob: Vec<u8> = vec![];

                let mut mode: Vec<u8> = String::from("100644").as_bytes().to_owned();
                let mut empty_string = String::from(" ").as_bytes().to_owned();
                let mut source = path_relative_to_cwd(blob.source()).as_bytes().to_owned();
                let mut null_byte: Vec<u8> = vec![0];
                let mut hex_id = hex::decode(blob.id()).expect("Invalid hex ID");

                binary_blob.append(&mut mode);
                binary_blob.append(&mut empty_string);
                binary_blob.append(&mut source);
                binary_blob.append(&mut null_byte);
                binary_blob.append(&mut hex_id);

                binary_blob
            })
            .flatten()
            .collect();
        Tree { content, blobs }
    }
}

fn path_relative_to_cwd(path: &PathBuf) -> String {
    let current_dir = env::current_dir()
        .expect("Couldn't determine current directory")
        .to_str()
        .unwrap()
        .to_owned();

    path.to_str()
        .unwrap()
        .to_owned()
        .replace(&current_dir, "")
        .trim_start_matches('/')
        .to_string()
}

impl Object for Tree {
    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn kind(&self) -> &Kind {
        &Kind::Tree
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;
    use crate::objects::blob::Blob;

    #[test]
    fn creates_from_blobs() {
        let blob = Blob {
            content: "Hello".to_string().as_bytes().to_owned(),
            source: PathBuf::from("wat.txt"),
        };
        let _tree = Tree::from_blobs(vec![blob]);
    }
}
