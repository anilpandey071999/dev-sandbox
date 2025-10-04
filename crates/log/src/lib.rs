use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    sync::{RwLock, mpsc::Receiver},
};
/*
 * `buffer writer` first keep every log in the memory buffer before writing to file this will make few system call
 */
pub struct Logger {
    rx: Receiver<String>,
    buf_file_writer: BufWriter<File>,
}

pub static CURRENT_FILE_SIZE: RwLock<u64> = RwLock::new(0);
pub const MAX_FILE_SIZE: u64 = 0;

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
            // current_size: 0,
            // max_size: 1_000_000_00,
        }
    }

    pub fn append(&mut self, log: String) -> () {
        let bytes_message = log.as_bytes();
        if *CURRENT_FILE_SIZE.read().unwrap() >= MAX_FILE_SIZE {
            *CURRENT_FILE_SIZE.write().unwrap() = 0;
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
        *CURRENT_FILE_SIZE.write().unwrap() += bytes_message.len() as u64;
    }

    pub fn run(&mut self) {
        println!("String the log reciving...");

        while let Ok(message) = self.rx.recv() {
            // self.append(message);
            info!(message);
        }

        println!("Logger stopped: channel closed.");
    }
}

#[macro_export]
macro_rules! info {
    ($buffer: expr) => {
        let bytes_message = stringify!($loger).as_bytes();
        let path = module_path!();
        if *crate::CURRENT_FILE_SIZE.read().unwrap() >= crate::MAX_FILE_SIZE {
            *crate::CURRENT_FILE_SIZE.write().unwrap() = 0;
            let file_path = format!("./logs/{}.log", chrono::Utc::now());
            let file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .expect("Failed to open file for appending");
            $buffer.buf_file_writer = BufWriter::new(file);
        }
        buf_file_writer
            .write_all(bytes_message)
            .expect("failed to append ");
        *crate::CURRENT_FILE_SIZE.write().unwrap() += bytes_message.len() as u64;
        println!("Hello Macro ! {:?} => {}", stringify!($loger), $loger);
    };
}
