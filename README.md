# marukov
a dead simple markov text generation library.

## usage
```rust
use marukov::{Text, TextOptions};

// load your data from somewhere
// each new line in your data is a new entry.
let lyrics = std::fs::read_to_string("weathergirl.txt").unwrap();

// create the text model
let text = Text::new(lyrics);

// start generating stuff
for _ in 0..5 {
    println!("{}", text.generate(TextOptions::default()))
}
```

### outputs example
```
ability so specific it sounds so stupid
asriel dreemurr from touhou 93
banished back to middle east
```
completely new sentence btw, you just have to give it a lot of data.