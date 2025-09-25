use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::{Arc, RwLock},
};

use crate::KvStore;

type KvSafeType<T, U> = Arc<RwLock<KvStore<T, U>>>;
pub struct KvHandle<T, U>
where
    T: Eq + Hash + Debug + Display,
    U: Debug + Display,
{
    pub store: KvSafeType<T, U>,
}

impl<T, U> KvHandle<T, U>
where
    T: Eq + Hash + Debug + Display,
    U: Debug + Display,
{
    pub fn new() -> Self {
        let kv_store: Arc<RwLock<KvStore<T, U>>> = Arc::new(RwLock::new(KvStore::new()));
        Self { store: kv_store }
    }
}
