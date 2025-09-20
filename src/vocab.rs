use std::collections::HashMap;

#[derive(Debug)]
pub struct Vocab {
    pub word_to_id: HashMap<String, u32>,
    pub id_to_word: Vec<String>,
}

impl Vocab {
    pub fn new() -> Self {
        Self {
            word_to_id: HashMap::new(),
            id_to_word: Vec::new(),
        }
    }

    pub fn get_or_insert(&mut self, word: &str) -> u32 {
        if let Some(&id) = self.word_to_id.get(word) {
            id
        } else {
            let id = self.id_to_word.len() as u32;
            self.word_to_id.insert(word.to_string(), id);
            self.id_to_word.push(word.to_string());
            id
        }
    }
}
