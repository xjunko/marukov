use crate::chain::Chain;
use regex::Regex;

const MOR: f32 = 0.7; // max overlap ratio
const MOT: usize = 15; // max overlap total

/// Options for generating text.
#[derive(Debug)]
pub struct TextOptions {
    pub tries: i32,
    pub min_words: i32,
    pub max_words: i32,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            tries: 999,
            min_words: 0,
            max_words: 100,
        }
    }
}

/// Text is the main structure for generating text based on a Markov model.
#[derive(Debug, Default)]
pub struct Text {
    reject: Option<Regex>,
    parsed_sentences: Vec<Vec<String>>,
    rejoined_text: String,
    chain: Chain,
}

impl Text {
    /// Validates the input sentence.
    fn sentence_input(&self, s: &str) -> bool {
        if s.trim().is_empty() {
            return false;
        }
        let decoded = unidecode::unidecode(s);
        if let Some(re) = &self.reject
            && re.is_match(&decoded)
        {
            return false;
        }
        true
    }

    /// Verifies that the generated words do not overlap significantly with the original text.
    fn verify(&self, words: &[String], mor: f32, mot: usize) -> bool {
        let overlap_ratio = (mor * words.len() as f32).round() as usize;
        let overlap_max = mot.min(overlap_ratio);
        let overlap_over = overlap_max + 1;
        let gram_count = (words.len().saturating_sub(overlap_max)).max(1);

        for i in 0..gram_count {
            let end = (i + overlap_over).min(words.len());
            let gram = &words[i..end];
            let gram_joined = gram.join(" ");
            if self.rejoined_text.contains(&gram_joined) {
                return false;
            }
        }

        true
    }

    /// Parses the input data into sentences.
    fn parse(&self, data: String) -> Vec<Vec<String>> {
        data.split("\n")
            .filter(|s| self.sentence_input(s))
            .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
            .collect()
    }
}

impl Text {
    /// Creates a new Text instance from the given data.
    ///
    /// # Arguments
    /// * `data` - A string containing the text data to be processed.
    /// # Returns
    /// A new instance of `Text`.
    pub fn new(data: String) -> Self {
        let mut text = Text::default();
        text.reject = Regex::new(&format!(r"(^')|('$)|\s'|'\s|[\{}(\(\)\[\])]", '"')).ok();
        text.parsed_sentences = text.parse(data);
        text.rejoined_text = text
            .parsed_sentences
            .iter()
            .map(|s| s.join(" "))
            .collect::<Vec<String>>()
            .join(" ");
        text.chain = Chain::new(&text.parsed_sentences);

        text
    }

    /// Generates text based on the Markov model and the provided options.
    ///
    /// # Arguments
    /// * `options` - A `TextOptions` struct containing parameters for text generation.
    /// # Returns
    /// A string containing the generated text.
    pub fn generate(&self, options: TextOptions) -> String {
        for _ in 0..options.tries {
            let words: Vec<String> = self.chain.generate(None);
            if words.len() > options.max_words as usize || words.len() < options.min_words as usize
            {
                continue;
            }
            if self.verify(&words, MOR, MOT) {
                return words.join(" ");
            }
        }

        String::new()
    }
}
