use kv::KvStore;

fn main() {
    let mut kv_store: KvStore<_, _> = KvStore::new();
    kv_store.set("key".to_string(), "value".to_string());
    kv_store.get(&"key".to_string()).unwrap();
}
