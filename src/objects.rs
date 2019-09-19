use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use sha1;
use uuid::Uuid;

use crate::compression;

pub enum Kind {
    Blob,
    // Tree,
}

use Kind::*;

pub struct Object {
    content: String,
    kind: Kind,
}

impl Object {
    pub fn blob_from_file<P: Into<PathBuf>>(path: P) -> Object {
        let content = fs::read_to_string(path.into()).expect("Couldn't read the file");
        Object {
            content,
            kind: Blob,
        }
    }

    // pub fn tree_from_blobs(blobs: Vec<Object>) -> Object {
    //     Object {
    //         content: "".to_string(),
    //         kind: Tree,
    //     }
    // }

    pub fn bytes(&self) -> Vec<u8> {
        let byte_length = self.content.len();
        let kind = match &self.kind {
            Blob => "blob",
            // Tree => "tree",
        };
        format!("{} {}\0{}", kind, byte_length, self.content)
            .as_bytes()
            .to_owned()
    }

    pub fn id(&self) -> String {
        let mut hash = sha1::Sha1::new();
        hash.update(self.bytes().as_slice());
        hash.digest().to_string()
    }

    pub fn path(&self) -> PathBuf {
        let current_dir = env::current_dir().expect("Couldn't get current working directory");
        let id = self.id();
        current_dir
            .join(".git")
            .join("objects")
            .join(&id[0..2])
            .join(&id[2..])
    }

    pub fn write(&self) -> Result<(), io::Error> {
        let path_to_object = self.path();
        let directory = path_to_object
            .parent()
            .expect("Couldn't get parent directory for object");
        let tmp_object_path = directory.join(format!("tmp_object_{}", Uuid::new_v4()));
        fs::create_dir_all(directory)?;
        let encoded_data = compression::compress(&self.bytes());
        fs::File::create(&tmp_object_path)?.write_all(&encoded_data)?;
        fs::rename(tmp_object_path, path_to_object)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_entry_from_contents() {
        let content = "This is some great content".to_string();
        let object = Object {
            content,
            kind: Blob,
        };
        assert_eq!(
            String::from_utf8(object.bytes()).unwrap(),
            "blob 26\0This is some great content".to_string()
        );
    }

    #[test]
    fn calculates_sha1_id_from_entry() {
        let content = "This is some great content".to_string();
        let object = Object {
            content,
            kind: Blob,
        };
        assert_eq!(
            object.id(),
            "a8be488abce200ee4f988c2a63ed5a61f8362521".to_string()
        );
    }

    #[test]
    fn knows_where_to_save_from_id() {
        let content = "This is some great content".to_string();
        let object = Object {
            content,
            kind: Blob,
        };

        // Just showing that we know the full SHA1 hash to base expected path from
        assert_eq!(
            object.id(),
            "a8be488abce200ee4f988c2a63ed5a61f8362521".to_string()
        );

        // first two characters of sha1
        let directory = "a8".to_string();
        // remaining characters of sha1
        let file = "be488abce200ee4f988c2a63ed5a61f8362521".to_string();

        let expected_path = env::current_dir()
            .unwrap()
            .join(".git")
            .join("objects")
            .join(directory)
            .join(file);

        assert_eq!(object.path(), expected_path);
    }

}
