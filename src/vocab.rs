use std::collections::HashMap;

#[derive(Debug)]
pub struct Vocab {
    word_to_id: HashMap<String, u32>,
    id_to_word: Vec<String>,
}

impl Default for Vocab {
    fn default() -> Self {
        Self::new()
    }
}

impl Vocab {
    /// Creates a new, empty vocabulary.
    pub fn new() -> Self {
        Self {
            word_to_id: HashMap::new(),
            id_to_word: Vec::new(),
        }
    }

    /// Converts a word to its corresponding token ID.
    /// If the word is not present in the vocabulary, it is added.
    /// For retrieving token IDs without adding new words, use `to_token_opt`.
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

    /// Converts a word to its corresponding token ID, returning None if not found.
    pub fn to_token_opt(&self, word: &str) -> Option<u32> {
        self.word_to_id.get(word).cloned()
    }

    /// Converts a token ID back to its corresponding word.
    pub fn to_word(&self, token: u32) -> &str {
        self.id_to_word
            .get(token as usize)
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}
