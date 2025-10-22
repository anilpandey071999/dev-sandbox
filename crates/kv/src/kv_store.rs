use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};
/*
 * key: T
 * value: U
 * T using Eq + Hash as rust compiler is not able to infer the type of T
 */
pub struct KvStore<T: Eq + Hash + Debug + Display, U: Debug + Display> {
    pub kv_store: HashMap<T, U>,
}

impl<T: Eq + Hash + Debug + Display, U: Debug + Display> KvStore<T, U> {
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
