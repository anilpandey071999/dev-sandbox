use kv::KvHandle;
use log::Logger;
use std::thread;

fn main() {
    let kv_handle = KvHandle::new();
    let (tx, rx) = std::sync::mpsc::channel();

    let mut logger = Logger::new(rx);
    let logger_thread = thread::spawn(move || {
        logger.run();
    });

    let kv_store_thread1 = kv_handle.store.clone();
    let tx_thread1 = tx.clone();

    let kv_store_thread2 = kv_handle.store.clone();
    let tx_thread2 = tx.clone();

    let thread_handler1 = thread::spawn(move || {
        for i in 0..10 {
            let key = format!("Key{i}");
            let value = format!("value{i}");
            let _ = tx_thread1
                .send(format!(
                    "thread1 iter count {i} | Key : {key} Value: {value} "
                ))
                .unwrap();
            kv_store_thread1.write().unwrap().set(key, value);
        }
    });

    let thread_handler2 = thread::spawn(move || {
        for i in 0..10 {
            let key = format!("Key{i}");
            let value = format!("value{i}");
            let _ = tx_thread2
                .send(format!(
                    "thread2 iter count {i} | Key : {key} Value: {value} "
                ))
                .unwrap();
            kv_store_thread2.write().unwrap().set(key, value);
        }
    });

    thread_handler1.join().unwrap();
    thread_handler2.join().unwrap();

    drop(tx);

    logger_thread.join().unwrap();

    println!("We have reached to end!!");
}
