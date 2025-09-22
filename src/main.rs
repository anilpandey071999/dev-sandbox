use kv::KvStore;

fn main() {
    // let mut kv_store: KvStore<_, _> = KvStore::new();
    // kv_store.set("key".to_string(), "value".to_string());
    // kv_store.get(&"key".to_string()).unwrap();
    let thread1 = std::thread::spawn(move ||{
        let mut kv_store: KvStore<_, _> = KvStore::new();
        for _ in 0..10 {
            kv_store.set("key1".to_string(), "value1".to_string());
        }
    });
    
    let thread2 = std::thread::spawn(move ||{
        let mut kv_store: KvStore<_, _> = KvStore::new();
        for _ in 0..10 {
            kv_store.set("key2".to_string(), "value2".to_string());
        }
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
}
