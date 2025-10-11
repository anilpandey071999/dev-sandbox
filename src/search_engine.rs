use std::collections::HashMap;

pub struct SearchEngine {
    pub search: HashMap<String, Vec<usize>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            search: HashMap::new(),
        }
    }

    pub fn insert_hashmap(&mut self, k: String, v: usize) {
        match self.search.insert(k, vec![v]) {
            Some(_) => println!("Updated successful 💕"),
            None => println!("Insered New Value 🚀"),
        }
        println!("Hash current len{}", self.search.len());
    }

    pub fn search_engine(&self) {}
}
