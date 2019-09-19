use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use sha1;
use uuid::Uuid;

use crate::compression;

pub struct Object {
    id: String,
    _contents: String,
    entry: String,
}

fn entry_from_contents(contents: &String) -> String {
    let object_type = String::from("blob");
    let byte_length = contents.to_owned().into_bytes().len().to_string();
    format!("{} {}\0{}", object_type, byte_length, contents)
}

pub fn id_from_entry(entry: &String) -> String {
    let mut m = sha1::Sha1::new();
    m.update(&entry.to_owned().into_bytes());
    m.digest().to_string()
}

impl Object {
    pub fn from_contents(contents: String) -> Object {
        let entry = entry_from_contents(&contents);
        let id = id_from_entry(&entry);
        Object {
            id,
            _contents: contents,
            entry,
        }
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> Object {
        let contents = fs::read_to_string(path.into()).expect("Couldn't read the file");
        Object::from_contents(contents)
    }

    pub fn path(&self) -> PathBuf {
        let current_dir = env::current_dir().expect("Couldn't get current working directory");
        let directory = current_dir
            .join(".git")
            .join("objects")
            .join(&self.id[0..2]);
        let path_to_file = directory.join(&self.id[2..]);
        path_to_file
    }

    pub fn write(&self) -> Result<(), io::Error> {
        let path_to_entry = self.path();
        let directory = path_to_entry
            .parent()
            .expect("Couldn't get parent directory for db object");
        let tmp_object_path = directory.join(format!("tmp_object_{}", Uuid::new_v4()));
        fs::create_dir_all(directory)?;
        let encoded_data = compression::compress(&self.entry.to_owned().as_bytes().to_vec());
        fs::File::create(&tmp_object_path)?.write_all(&encoded_data)?;
        fs::rename(tmp_object_path, path_to_entry)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_entry_from_contents() {
        let contents = "This is some great content".to_string();
        let object = Object::from_contents(contents);
        assert_eq!(
            object.entry,
            "blob 26\0This is some great content".to_string()
        );
    }

    #[test]
    fn calculates_sha1_id_from_entry() {
        let contents = "This is some great content".to_string();
        let object = Object::from_contents(contents);
        assert_eq!(
            object.id,
            "a8be488abce200ee4f988c2a63ed5a61f8362521".to_string()
        );
    }

    #[test]
    fn knows_where_to_save_from_id() {
        let contents = "This is some great content".to_string();
        let object = Object::from_contents(contents);

        // Just showing that we know the full SHA1 hash to base expected path from
        assert_eq!(
            object.id,
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
