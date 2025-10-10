use chrono::Utc;
use lazy_static::lazy_static;
use std::{
    fs::{self, File},
    io::BufWriter,
    sync::{RwLock, mpsc::Receiver},
};

/*
 * `buffer writer` first keep every log in the memory buffer before writing to file this will make few system call
 */
pub struct Logger {
    rx: Receiver<String>,
    // _buf_file_writer: BufWriter<File>,
}

lazy_static! {
    pub static ref BUFFER_WRITER: RwLock<BufWriter<File>> = {
        if fs::read_dir("./logs").is_err() {
            fs::create_dir("./logs").expect("Failed to create logs directory");
        }
        let file_name = format!("logs/{}.log", Utc::now().format("%Y-%m-%dT%H-%M-%S"));
        let file = File::create(&file_name).expect("Failed to create log file");
        RwLock::new(BufWriter::new(file))
    };
}
pub static CURRENT_FILE_SIZE: RwLock<u64> = RwLock::new(0);
pub const MAX_FILE_SIZE: u64 = 1_000_000_000;

impl Logger {
    pub fn new(rx: Receiver<String>) -> Self {
        if let Err(err) = fs::read_dir("./logs") {
            eprintln!("logs folder not found: {}", err);
            println!("Creating logs folder");
            if let Err(err) = fs::create_dir("./logs") {
                panic!("Failed to crate logs folder {}", err);
            }
        }
        Self { rx }
    }

    // pub fn append(&mut self, log: String) -> () {
    //     let bytes_message = log.as_bytes();
    //     if *CURRENT_FILE_SIZE.read().unwrap() >= MAX_FILE_SIZE {
    //         *CURRENT_FILE_SIZE.write().unwrap() = 0;
    //         let file_path = format!("./logs/{}.log", chrono::Utc::now());
    //         let file = fs::OpenOptions::new()
    //             .append(true)
    //             .create(true)
    //             .open(file_path)
    //             .expect("Failed to open file for appending");
    //         self.buf_file_writer = BufWriter::new(file);
    //     }
    //     self.buf_file_writer
    //         .write_all(bytes_message)
    //         .expect("failed to append ");
    //     *CURRENT_FILE_SIZE.write().unwrap() += bytes_message.len() as u64;
    // }

    pub fn run(&mut self) {
        println!("String the log reciving...");

        while let Ok(message) = self.rx.recv() {
            info!(message);
        }

        println!("Logger stopped: channel closed.");
    }
}

#[macro_export]
macro_rules! info {
    ($message: expr) => {
        use std::io::Write;
        let mut writer = $crate::BUFFER_WRITER.write().unwrap();
        let message = format!("{} {}", module_path!(), $message);

        let mut size = $crate::CURRENT_FILE_SIZE.write().unwrap();
        if *size >= $crate::MAX_FILE_SIZE {
            *size = 0;
            let file_name = format!(
                "logs/{}.log",
                chrono::Utc::now().format("%Y-%m-%dT%H-%M-%S")
            );
            let file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_name)
                .expect("Failed to open file for appending");
            *writer = std::io::BufWriter::new(file);
        }
        writer
            .write_all(message.as_bytes())
            .expect("failed to append");
        *size += message.len() as u64;
    };
}
