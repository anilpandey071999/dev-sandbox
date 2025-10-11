use crate::search_engine::SearchEngine;

#[derive(Debug, Clone)]
pub struct Document<'a> {
    pub title: &'a str,
    pub contents: &'a str,
}

/// postion of doument is the Document's ID
#[derive(Debug)]
pub struct Documents<'a> {
    pub docs: Vec<Document<'a>>,
}

impl<'a> Documents<'a> {
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    pub fn insert_documents(&mut self, title: &'a str, contents: &'a str, hash: &mut SearchEngine) {
        let document = Document { title, contents };
        self.docs.push(document.clone());
        hash.insert_full_content(title, contents, self.docs.len() - 1);
        println!("{:?} \n {:?}", document, hash.search);
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
