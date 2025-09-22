use std::{collections::HashMap, hash::Hash};


pub struct KvStore<T: Eq + Hash, U> {
    pub kv_store: HashMap<T, U>,
}

impl<T: Eq + Hash, U> KvStore<T, U> {
    pub fn new() -> Self {
        Self {
            kv_store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: T, value: U) -> Option<U> {
        self.kv_store.insert(key, value)
    }
    
    pub fn get(&self, key: &T) -> Option<&U> {
       self.kv_store.get(key)
    }
}
