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

    pub fn search_engine(&self) {
        
    }
}
