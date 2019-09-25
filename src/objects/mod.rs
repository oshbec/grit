use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use sha1;
use uuid::Uuid;

mod blob;
mod tree;

pub use blob::Blob;
pub use tree::Tree;

use crate::compression;

pub enum Kind {
    Blob,
    Tree,
    // Commit,
}

use Kind::*;

pub trait Object {
    fn content(&self) -> &Vec<u8>;
    fn kind(&self) -> &Kind;

    fn id(&self) -> String {
        let mut hash = sha1::Sha1::new();
        let content = self.content().as_slice();
        hash.update(content);
        hash.digest().to_string()
    }

    fn path(&self) -> PathBuf {
        let current_dir = env::current_dir().expect("Couldn't get current working directory");
        let id = self.id();
        current_dir
            .join(".git")
            .join("objects")
            .join(&id[0..2])
            .join(&id[2..])
    }

    fn build_file(&self) -> Vec<u8> {
        let kind = match self.kind() {
            Blob => "blob",
            Tree => "tree",
            // Commit => "commit",
        };
        let byte_length = self.content().len();

        let mut file: Vec<u8> = format!("{} {}\0", kind, byte_length).as_bytes().to_vec();
        file.append(&mut self.content().to_owned());
        file
    }

    fn show_file(&self) -> String {
        String::from_utf8_lossy(&self.build_file()).into_owned()
    }
}

pub fn write(object: &impl Object) -> Result<(), io::Error> {
    let compressed_data = compression::compress(&object.build_file());
    let path_to_object = object.path();

    let directory = path_to_object
        .parent()
        .expect("Couldn't get parent directory for object");
    fs::create_dir_all(directory)?;
    let tmp_object_path = directory.join(format!("tmp_object_{}", Uuid::new_v4()));

    fs::File::create(&tmp_object_path)?.write_all(&compressed_data)?;
    fs::rename(tmp_object_path, path_to_object)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SomeObject {
        content: Vec<u8>,
    }

    impl SomeObject {
        fn new() -> SomeObject {
            SomeObject {
                content: String::from("This is some great content")
                    .as_bytes()
                    .to_owned(),
            }
        }
    }

    impl Object for SomeObject {
        fn content(&self) -> &Vec<u8> {
            &self.content
        }

        fn kind(&self) -> &Kind {
            &Kind::Blob
        }
    }

    #[test]
    fn calculates_sha1_id_from_entry() {
        let object = SomeObject::new();
        assert_eq!(
            object.id(),
            "cc9d52752996f06ac0f1479f56f33440a6c6f2f8".to_string()
        );
    }

    #[test]
    fn knows_where_to_save_from_id() {
        let object = SomeObject::new();

        // Just showing that we know the full SHA1 hash to base expected path from
        assert_eq!(
            object.id(),
            "cc9d52752996f06ac0f1479f56f33440a6c6f2f8".to_string()
        );

        // first two characters of sha1
        let directory = "cc".to_string();
        // remaining characters of sha1
        let file = "9d52752996f06ac0f1479f56f33440a6c6f2f8".to_string();

        let expected_path = env::current_dir()
            .unwrap()
            .join(".git")
            .join("objects")
            .join(directory)
            .join(file);

        assert_eq!(object.path(), expected_path);
    }

}
