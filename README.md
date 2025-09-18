# marukov
a dead simple markov text generation library.

## usage
```rust
use marukov::{Text, TextOptions};

// load your data from somewhere
// each new line in your data is a new entry.
let lyrics = std::fs::read_to_string("weathergirl.txt").expect("shucks");

// create the text model
let text = Text::new(lyrics);

// start generating stuff
for _ in 0..5 {
    println!("{}", text.generate(TextOptions::default()))
}
```