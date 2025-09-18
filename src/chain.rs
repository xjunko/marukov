use std::collections::HashMap;

use rand::Rng;

const BEGIN: &str = "___BEGIN__";
const END: &str = "___END__";

const STATE_SIZE: usize = 2;

type State = Vec<String>;
type Weight = HashMap<String, i32>;
type Model = HashMap<State, Weight>;

/// Chain is used internally to generate text based on a Markov model.
#[derive(Debug, Default)]
pub struct Chain {
    model: Model,
    begin_choices: Vec<String>,
    begin_weights: Vec<i32>,
}

impl Chain {
    /// Accumulate a list of integers into a cumulative distribution.
    fn accumulate(ns: &[i32]) -> Vec<i32> {
        let mut numbers: Vec<i32> = Vec::with_capacity(ns.len());
        let mut total = ns[0];
        for &n in ns {
            numbers.push(total);
            total += n; // yes this is on purpose
        }
        numbers
    }

    /// Compile the next possible words and their cumulative weights.
    fn compile_next(data: &Weight) -> (Vec<String>, Vec<i32>) {
        let words: Vec<String> = data.keys().cloned().collect();
        let weights: Vec<i32> = data.values().cloned().collect();
        let cum: Vec<i32> = Self::accumulate(&weights);
        (words, cum)
    }

    /// Refer to python's `bisect.bisect`, this is more or less the same.
    fn bisect_right<T: Ord>(slice: &[T], x: &T) -> usize {
        match slice.binary_search(x) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    }
}

impl Chain {
    /// Creates a new Chain from the given data.
    ///
    /// # Arguments
    /// * `data` - A reference to a slice of vectors of strings, where each vector represents a sequence of words.
    /// # Returns
    /// A new instance of `Chain`.
    pub fn new(data: &[Vec<String>]) -> Self {
        let mut chain = Self::default();
        chain.model = chain.build(data);
        chain.compute();
        chain
    }

    /// Builds the Markov model from the provided data.
    fn build(&self, data: &[Vec<String>]) -> Model {
        let mut model: Model = HashMap::new();

        for run in data {
            let mut items: Vec<&str> = vec![BEGIN; STATE_SIZE];
            items.extend(run.iter().map(|s| s.as_str()));
            items.push(END);

            for i in 0..run.len() + 1 {
                let state: State = items[i..i + STATE_SIZE]
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect::<Vec<String>>();
                let follow: &str = items[i + STATE_SIZE];

                model
                    .entry(state)
                    .or_insert_with(HashMap::new)
                    .entry(follow.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        model
    }

    /// Returns the initial state of the Markov chain.
    fn begin_state(&self) -> State {
        vec![BEGIN.to_string(); STATE_SIZE]
    }

    /// Precomputes the choices and weights for the initial state.
    fn compute(&mut self) {
        let begin_state = self.begin_state();
        if let Some(weights) = self.model.get(&begin_state) {
            let (choices, cum) = Self::compile_next(weights);
            self.begin_choices = choices;
            self.begin_weights = cum;
        }
    }

    /// Moves to the next state based on the current state.
    ///
    /// # Arguments
    /// * `state` - A reference to the current state of the Markov chain.
    /// # Returns
    /// A string representing the next word in the sequence.
    pub fn next(&self, state: &State) -> String {
        let (mut choices, mut cumdist) = (self.begin_choices.clone(), self.begin_weights.clone());
        if state != &self.begin_state() {
            // FIXME: This is bad
            choices.clear();
            cumdist.clear();
            let mut weights: Vec<i32> = Vec::new();
            for (word, weight) in self.model.get(state).unwrap() {
                choices.push(word.clone());
                weights.push(*weight);
            }
            cumdist = Self::accumulate(&weights);
        }
        let r: f32 = rand::rng().random_range(0.0..1.0) * (*cumdist.last().unwrap() as f32);
        let r_i32 = r as i32;
        choices[Self::bisect_right(&cumdist, &r_i32)].clone()
    }

    /// Generates a sequence of words based on the Markov model.
    /// # Arguments
    /// * `init_state` - An optional initial state to start the generation from.
    /// # Returns
    /// A vector of strings representing the generated sequence of words.
    pub fn generate(&self, init_state: Option<State>) -> Vec<String> {
        let mut state = init_state.unwrap_or(self.begin_state());
        let mut result: Vec<String> = Vec::new();
        loop {
            let next_word: String = self.next(&state);
            if next_word == END {
                break;
            }
            result.push(next_word.clone());
            state.remove(0);
            state.push(next_word);
        }
        result
    }
}
