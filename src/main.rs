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
        for i in 0..1_000_000 {
            let key = format!("Key{i}");
            let value = format!("value{i}");
            let _ = tx_thread1
                .send(format!(
                    "thread1 iter count {i} | Key : {key} Value: {value} \n"
                ))
                .unwrap();
            kv_store_thread1.write().unwrap().set(key, value);
        }
    });

    let thread_handler2 = thread::spawn(move || {
        for i in 1_00_000..2_000_000 {
            let key = format!("Key{i}");
            let value = format!("value{i}");
            let _ = tx_thread2
                .send(format!(
                    "thread2 iter count {i} | Key : {key} Value: {value} \n"
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

// fn main(){
//     let count_rwlock = std::sync::Arc::new(std::sync::RwLock::new(0));
//     let falge_rwlock = std::sync::Arc::new(std::sync::RwLock::new(true));
    
//     let thread_count1_rwlock = count_rwlock.clone();
//     let thread_flage1_rwlock = falge_rwlock.clone();
    
//     let thread_count2_rwlock = count_rwlock.clone();
//     let thread_flage2_rwlock = falge_rwlock.clone();
    
//     let thread1 = thread::spawn(move ||{
//         *thread_flage1_rwlock.write().unwrap() = false;
//         for i in 0..1_00_00_000_00{
//             *thread_count1_rwlock.write().unwrap() += 1;
//             // println!("thread1 increamenting {i}");
//         } 
//         *thread_flage1_rwlock.write().unwrap() = true;
//         println!("Thread count thread 1 {}", thread_count1_rwlock.read().unwrap());
//     });
    
    
//     let thread2 = thread::spawn(move ||{
//         let mut count = 0;
//         while !*thread_flage2_rwlock.read().unwrap(){
//             count += 1;
//             println!("xxxxxxxxxxxxxxxxxxx count {count} decreamenting is waiting!!xxxxxxxxxxxxxxxxxxx");
//         }
//         for i in 0..1_00_00_000_00{
//             // println!("thread2 decreamenting {i}");
//             *thread_count2_rwlock.write().unwrap() -= 1;
//         } 
//     });
//     thread1.join().unwrap();
//     thread2.join().unwrap();
    
    
    
//     println!("Thread count {}", count_rwlock.read().unwrap());
// }