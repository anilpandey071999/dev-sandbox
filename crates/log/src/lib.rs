use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    sync::mpsc::Receiver,
};
/*
 * `buffer writer` first keep every log in the memory buffer before writing to file this will make few system call
 */
pub struct Logger {
    rx: Receiver<String>,
    buf_file_writer: BufWriter<File>,
    current_size: u64,
    max_size: u64,
}

impl Logger {
    pub fn new(rx: Receiver<String>) -> Self {
        if let Err(err) = fs::read_dir("./logs") {
            eprintln!("logs folder not found: {}", err);
            println!("Creating logs folder");
            if let Err(err) = fs::create_dir("./logs") {
                panic!("Failed to crate logs folder {}", err);
            }
        }

        let file_path = format!("./logs/{}.log", chrono::Utc::now());
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open file for appending");
        let buf_write = BufWriter::new(file);
        Self {
            rx,
            buf_file_writer: buf_write,
            current_size: 0,
            max_size: 1_000_000_00,
        }
    }

    pub fn append(&mut self, log: String) -> () {
        let bytes_message = log.as_bytes();

        if self.current_size >= self.max_size {
            self.current_size = 0;
            let file_path = format!("./logs/{}.log", chrono::Utc::now());
            let file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .expect("Failed to open file for appending");
            self.buf_file_writer = BufWriter::new(file);
        }
        self.buf_file_writer
            .write_all(bytes_message)
            .expect("failed to append ");
        self.current_size += bytes_message.len() as u64;
    }

    pub fn run(&mut self) {
        println!("String the log reciving...");

        while let Ok(message) = self.rx.recv() {
            self.append(message);
        }

        println!("Logger stopped: channel closed.");
    }
}
