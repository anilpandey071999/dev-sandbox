use crate::search_engine::SearchEngine;

#[derive(Debug)]
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

    pub fn insert_documents(
        &mut self,
        title: &'a str,
        contents: &'a str,
        hash: &mut SearchEngine<'a>,
    ) {
        let document = Document { title, contents };
        self.docs.push(document);
        hash.insert_full_content(title, contents, self.docs.len() - 1);
    }

    pub fn delete_documents(&mut self, id: usize) -> Result<(), String> {
        if self.docs.len() <= id {
            return Err("Invalide Document Id 🏴‍☠️".to_string());
        }
        let removed_doc = self.docs.remove(id);
        println!("{} has been deleted!!💀", removed_doc.title);
        Ok(())
    }

    pub fn search_key(&self, k: &str, hash: &SearchEngine) -> Result<Vec<&Document>, String> {
        if let Some(record) = hash.search_engine(k) {
            let mut docs = Vec::new();
            record.iter().for_each(|index| {
                docs.push(&self.docs[*index]);
            });
            return Ok(docs);
        }

        Err("No found 💀".to_string())
    }
}
