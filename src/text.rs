use crate::chain::Chain;
use crate::vocab::Vocab;
use regex::Regex;

const MOR: f32 = 0.7; // max overlap ratio
const MOT: usize = 15; // max overlap total

const BEGIN: &str = "___BEGIN__";
const END: &str = "___END__";

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
#[derive(Debug)]
pub struct Text {
    reject: Option<Regex>,
    parsed_sentences: Vec<Vec<u32>>,
    rejoined_text: String,
    chain: Chain<u32>,
    tokenizer: Vocab,
}

impl Text {
    /// Creates a default Text instance.
    /// Do **NOT** use this, use `Text::new` instead.
    fn default() -> Self {
        Self {
            reject: None,
            parsed_sentences: Vec::with_capacity(0),
            rejoined_text: String::with_capacity(0),
            chain: Chain::default(0, 0),
            tokenizer: Vocab::new(),
        }
    }

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
    fn parse(&mut self, data: String) -> (Vec<Vec<u32>>, String) {
        let sentences: Vec<&str> = data
            .split("\n")
            .filter(|s| self.sentence_input(s))
            .collect();

        let rejoined = sentences.to_vec().join(" ");

        (
            sentences
                .into_iter()
                .map(|s| {
                    s.split_whitespace()
                        .map(|w| self.tokenizer.to_token(w))
                        .collect()
                })
                .collect(),
            rejoined,
        )
    }
}

impl Text {
    /// Creates a new Text instance from the given data.
    /// # Arguments
    /// * `data` - A string containing the text data to be processed.
    /// # Returns
    /// A new instance of `Text`.
    pub fn new(data: String) -> Self {
        let mut text = Text::default();
        text.reject = Regex::new(&format!(r"(^')|('$)|\s'|'\s|[\{}(\(\)\[\])]", '"')).ok();
        (text.parsed_sentences, text.rejoined_text) = text.parse(data);
        text.chain = Chain::new(
            &text.parsed_sentences,
            text.tokenizer.to_token(BEGIN),
            text.tokenizer.to_token(END),
        );
        text
    }

    /// Generates text based on the Markov model and the provided options.
    /// # Arguments
    /// * `options` - A `TextOptions` struct containing parameters for text generation.
    /// # Returns
    /// A string containing the generated text.
    pub fn generate(&self, options: TextOptions) -> String {
        for _ in 0..options.tries {
            let tokens: Vec<u32> = self.chain.generate(None);
            if tokens.len() > options.max_words as usize
                || tokens.len() < options.min_words as usize
            {
                continue;
            }
            let words: Vec<String> = tokens
                .iter()
                .map(|&token| self.tokenizer.to_word(token).to_string())
                .collect();

            if self.verify(&words, MOR, MOT) {
                return words.join(" ");
            }
        }
        String::with_capacity(0)
    }
}
