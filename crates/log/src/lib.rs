use lazy_static::lazy_static;
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

lazy_static! {
    static ref file_name: String = format!("logs/{}", chrono::Utc::now());
    static ref BUFFER_WRITER: RwLock<BufWriter<File>> =
        RwLock::new(BufWriter::new(File::create(file_name.as_str()).unwrap()));
}

// static mut BUFFER_WRITER: RwLock<Option<BufWriter<File>>> = RwLock::new(None);
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
        // let file_path = format!("./logs/{}.log", chrono::Utc::now());
        // let file = std::fs::OpenOptions::new()
        //     .append(true)
        //     .create(true)
        //     .open(file_path)
        //     .expect("Failed to open file for appending");
        // let a = *BUFFER_WRITER.write().unwrap();
        // a =

        while let Ok(message) = self.rx.recv() {
            info!(message);
        }

        println!("Logger stopped: channel closed.");
    }
}

// #[macro_export]
// macro_rules! info {
//     ($message: expr) => {
//         // logger_writer!()
//     };
// }

#[macro_export]
macro_rules! info {
    ($message: expr) => {
        use crate::CURRENT_FILE_SIZE;
        // let buffer = Buf

        let mut buffer_writer = BUFFER_WRITER.write().unwrap();
        println!(
            "{} {}",
            *crate::CURRENT_FILE_SIZE.read().unwrap(),
            crate::MAX_FILE_SIZE
        );
        let bytes_message = format!("{} {}", module_path!(), $message);
        // let path = module_path!();
        if *crate::CURRENT_FILE_SIZE.read().unwrap() >= crate::MAX_FILE_SIZE {
            *crate::CURRENT_FILE_SIZE.write().unwrap() = 0;
            let file_path = format!("./logs/{}.log", chrono::Utc::now());
            let file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .expect("Failed to open file for appending");
            *buffer_writer = BufWriter::new(file);
        }
        buffer_writer
            .write_all(bytes_message.as_bytes())
            .expect("failed to append ");
        *crate::CURRENT_FILE_SIZE.write().unwrap() += bytes_message.len() as u64;
        println!("Hello Macro ! {:?} => {}", stringify!($loger), $message);
    };
}
