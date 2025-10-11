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
        let _ = self
            .search
            .entry(k)
            .and_modify(|val| {
                if !val.contains(&v) {
                    val.push(v);
                }
            })
            .or_insert(vec![v]);
        println!("Hash current len{}", self.search.len());
    }

    pub fn insert_full_content(&mut self, title: &str, content: &str, v: usize) {
        title
            .split_whitespace()
            .for_each(|word| self.insert_hashmap(word.to_string(), v));
        println!("{:?}", self.search);
        let _ = content
            .split_whitespace()
            .for_each(|word| self.insert_hashmap(word.to_string(), v));
    }

    pub fn search_engine(&self) {}
}
