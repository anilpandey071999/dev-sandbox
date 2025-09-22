use std::{collections::HashMap, fmt::{Debug, Display}, hash::Hash, time::{self, SystemTime}};
use log::{self, Logger};
/*
 * key: T
 * value: U
 * T using Eq + Hash as rust compiler is not able to infer the type of T
 */
pub struct KvStore<T: Eq + Hash + Debug + Display, U: Debug + Display> {
    pub kv_store: HashMap<T, U>,
    logger: log::Logger,
}

impl<T: Eq + Hash + Debug + Display + Clone, U: Debug + Display + Clone> KvStore<T, U> {
    pub fn new() -> Self {
        Self {
            kv_store: HashMap::new(),
            logger: Logger::new()
        }
    }

    pub fn set(&mut self, key: T, value: U) -> bool {
        self.kv_store.insert(key.clone(), value.clone());
        self.logger.append(format!("{:?} key: {:?}, value: {:?} \n", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(), key, value));
        true
    }
    
    pub fn get(&self, key: &T) -> Option<&U> {
        if let Some(value) = self.kv_store.get(key) {
            Some(value)
        } else {
            None
        }
    }
}
