use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;

pub const STATE_SIZE: usize = 2;

pub type State<T> = Vec<T>;
pub type Weight<T> = HashMap<T, i32>;
pub type Model<T> = HashMap<State<T>, Weight<T>>;

/// Chain is used internally to generate text based on a Markov model.
#[derive(Debug)]
pub struct Chain<T>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
{
    token_begin: T,
    token_end: T,
    model: Model<T>,
    begin_choices: Vec<T>,
    begin_weights: Vec<i32>,
}

impl<T> Chain<T>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
{
    /// Creates an empty Chain.
    pub fn default(begin: T, end: T) -> Self {
        Self {
            token_begin: begin,
            token_end: end,
            model: Model::new(),
            begin_choices: Vec::new(),
            begin_weights: Vec::new(),
        }
    }

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
    fn compile_next(data: &Weight<T>) -> (Vec<T>, Vec<i32>) {
        let words: Vec<T> = data.keys().cloned().collect();
        let weights: Vec<i32> = data.values().cloned().collect();
        let cum: Vec<i32> = Self::accumulate(&weights);
        (words, cum)
    }

    /// Refer to python's `bisect.bisect`, this is more or less the same.
    fn bisect_right<X: Ord>(slice: &[X], x: &X) -> usize {
        match slice.binary_search(x) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    }
}

impl<T> Chain<T>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
{
    /// Creates a new Chain from the given data.
    /// # Arguments
    /// * `data` - A reference to a slice of vectors of strings, where each vector represents a sequence of words.
    /// # Returns
    /// A new instance of `Chain`.
    pub fn new(data: &[Vec<T>], begin: T, end: T) -> Self {
        let mut chain = Self::default(begin, end);
        chain.model = chain.build(data);
        chain.compute();
        chain
    }

    /// Builds the Markov model from the provided data.
    fn build(&self, data: &[Vec<T>]) -> Model<T> {
        let mut model: Model<T> = HashMap::new();

        for run in data {
            let mut items: Vec<&T> = vec![&self.token_begin; STATE_SIZE];
            items.extend(run);
            items.push(&self.token_end);

            for i in 0..run.len() + 1 {
                let state: State<T> = items[i..i + STATE_SIZE].iter().cloned().cloned().collect();
                let follow: &T = items[i + STATE_SIZE];

                model
                    .entry(state)
                    .or_default()
                    .entry(follow.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        model
    }

    /// Returns the initial state of the Markov chain.
    fn begin_state(&self) -> State<T> {
        vec![self.token_begin.clone(); STATE_SIZE]
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
    /// # Arguments
    /// * `state` - A reference to the current state of the Markov chain.
    /// # Returns
    /// A <T> representing the next token in the sequence.
    pub fn next(&self, state: &State<T>) -> T {
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
    pub fn generate(&self, init_state: Option<State<T>>) -> Vec<T> {
        let mut state = init_state.unwrap_or(self.begin_state());
        let mut result: Vec<T> = Vec::new();

        loop {
            let next_word: T = self.next(&state);
            if next_word == self.token_end {
                break;
            }
            result.push(next_word.clone());
            state.remove(0);
            state.push(next_word);
        }
        result
    }

    /// Finds an initial state containing the specified start token.
    /// # Arguments
    /// * `start` - The token to search for in the initial states.
    /// # Returns
    /// An optional vector of states containing the start token.
    pub fn find_init_states(&self, start: T) -> Option<Vec<State<T>>> {
        self.model
            .keys()
            .filter(|state| state.contains(&start))
            .cloned()
            .collect::<Vec<State<T>>>()
            .into()
    }
}
