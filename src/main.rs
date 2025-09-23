use kv::KvStore;
use log::Logger;
use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let kv_store = Arc::new(RwLock::new(KvStore::new()));
    let (tx, rx) = std::sync::mpsc::channel();
    let mut logs = Logger::new(rx);

    let logger_thread = thread::spawn(move || {
        logs.run();
    });

    let kv_thread1 = kv_store.clone();
    let tx_thread1 = tx.clone();
    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            // let a = kv_thread1.get_mut().unwrap();
            if let Err(send_log_err) =
                tx_thread1.send(format!("key: {} value: {}", "key1", "value1"))
            {
                eprintln!("Error sending log: {}", send_log_err);
            }
            kv_thread1
                .write()
                .unwrap()
                .set("key1".to_string(), "value1".to_string());
            // kv_store.set("key1".to_string(), "value1".to_string());
        }
    });

    let kv_thread2 = kv_store.clone();
    let tx_thread2 = tx.clone();
    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            if let Err(send_log_err) =
                tx_thread2.send(format!("key: {} value: {}", "key2", "value2"))
            {
                eprintln!("Error sending log: {}", send_log_err);
            }
            kv_thread2
                .write()
                .unwrap()
                .set("key2".to_string(), "value2".to_string());
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    drop(tx);

    logger_thread.join().unwrap();

    println!("We have reached to end!!");
}
