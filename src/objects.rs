use sha1;
use std::{env, fs, io, path::PathBuf};

use uuid::Uuid;

use std::io::Write;

use libflate;

pub struct Object {
    id: String,
    contents: String,
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
            contents,
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
        let mut tmp_object_file = fs::File::create(&tmp_object_path)?;
        let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
        io::copy(&mut self.entry.to_owned().as_bytes(), &mut encoder).unwrap();
        let encoded_data = encoder
            .finish()
            .into_result()
            .expect("Couldn't get encoding result");
        tmp_object_file.write_all(&encoded_data)?;
        fs::rename(tmp_object_path, path_to_entry)?;
        Ok(())
    }
}
