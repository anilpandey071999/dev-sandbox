use std::{
    fs::{self, File},
    io::Write,
    sync::mpsc::Receiver,
};

pub struct Logger {
    rx: Receiver<String>,
    file: File,
}

impl Logger {
    pub fn new(rx: Receiver<String>) -> Self {
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("./logs/1.log")
            .expect("Failed to open file for appending");

        Self { rx, file }
    }

    pub fn append(&mut self, log: String) -> () {
        self.file
            .write_all(log.as_bytes())
            .expect("failed to append ")
    }
    
    pub fn run(&mut self) {
        println!("String the log reciving...");
        
        while let Ok(message) = self.rx.recv() {
            let log_entry = if message.ends_with("\n"){
                message
            }else{
                format!("{}\n",message)
            };
            
            if let Err(e) = self.file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write log: {}", e);
            } else {
                let _ = self.file.flush();
            }
        }
        
        println!("Logger stopped: channel closed.");
    }
}
