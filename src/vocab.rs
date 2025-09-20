use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Vocab {
    word_to_id: HashMap<String, u32>,
    id_to_word: Vec<String>,
}

impl Vocab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_token(&mut self, word: &str) -> u32 {
        if let Some(&id) = self.word_to_id.get(word) {
            return id;
        }

        let id = self.id_to_word.len() as u32;
        self.id_to_word.push(word.to_owned());
        let inserted = self.id_to_word.last().unwrap().clone();
        self.word_to_id.insert(inserted, id);

        id
    }

    pub fn to_word(&self, token: u32) -> &str {
        self.id_to_word
            .get(token as usize)
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}
