use std::collections::HashMap;

pub struct SearchEngine<'search> {
    pub search: HashMap<&'search str, Vec<usize>>,
}

impl<'search> SearchEngine<'search> {
    pub fn new() -> Self {
        Self {
            search: HashMap::new(),
        }
    }

    pub fn insert_hashmap(&mut self, k: &'search str, v: usize) {
        let _ = self
            .search
            .entry(k)
            .and_modify(|val| {
                if !val.contains(&v) {
                    val.push(v);
                }
            })
            .or_insert(vec![v]);
        // println!("Hash current len{}", self.search.len());
    }

    pub fn insert_full_content(&mut self, title: &'search str, content: &'search str, v: usize) {
        title
            .split_whitespace()
            .for_each(|word| self.insert_hashmap(word, v));
        // println!("{:?}", self.search);
        let _ = content
            .split_whitespace()
            .for_each(|word| self.insert_hashmap(word, v));
    }

    pub fn search_engine(&self, k: &str) -> Option<&Vec<usize>> {
        self.search.get(k.to_lowercase().as_str())
    }
}
