use std::{fs::{self, File}, io::Write};

pub struct Logger {
    pub log: String,
    file: File,
}

impl Logger {
    pub fn new() -> Self {
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("./logs/1.log")
            .expect("Failed to open file for appending");

        Self {
            log: String::new(),
            file,
        }
    }

    pub fn append(&mut self, log: String) -> () {
        self.file.write_all(log.as_bytes()).expect("failed to append log!!")
    }
}
