use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub trait FileSystemOperations {
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error>;
    fn write_file(&mut self, file_path: &str, content: &str) -> Result<(), std::io::Error>;
    fn exists(&self, file_path: &str) -> bool;
}

pub struct FileSystem;

impl FileSystemOperations for FileSystem {
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn exists(&self, file_path: &str) -> bool {
        std::fs::metadata(file_path).is_ok()
    }

    fn write_file(&mut self, file_path: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
