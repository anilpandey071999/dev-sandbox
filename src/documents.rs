use crate::search_engine::SearchEngine;

pub struct Document<'a> {
    pub title: &'a str,
    pub contents: &'a str,
}

/// postion of doument is the Document's ID
pub struct Documents<'a> {
    pub docs: Vec<Document<'a>>,
}

impl<'a> Documents<'a> {
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    pub fn insert_documents(&mut self, title: &'a str, contents: &'a str, hash: &mut SearchEngine) {
        let document = Document { title, contents };
        let a = self.docs.len();
        hash.insert_hashmap(title.to_string(), a);
        self.docs.push(document);
    }

    pub fn delete_documents(&mut self, id: usize) -> Result<(), String> {
        if self.docs.len() <= id {
            return Err("Invalide Document Id 🏴‍☠️".to_string());
        }
        let removed_doc = self.docs.remove(id);
        println!("{} has been deleted!!💀", removed_doc.title);
        Ok(())
    }
}
