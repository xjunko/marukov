# marukov
a very simple markov-chain related library, mostly used for text generations but can be applied to anything else.

## features
- `Chain` can be re-used in other applications, refer to [[here]](https://en.wikipedia.org/wiki/Markov_chain#Applications).
- `Text` generations are done with GPT-like tokenizer, which speds things up significantly.
- Made with text generation in mind, and so, it's really good at doing that.

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
    if let Some(res) = text.generate(TextOptions::default()) {
        println("{}", res);
    }
}

// or if you want to generate starting from a word
for _ in 0..5 {
    if let Some(res) = text.generate_with_start("uma", TextOptions::default()) {
        println("{}", res);
    }
}
```

### outputs example
```
uma musume fans when your take a visit to uniqlo
ability so specific it sounds so stupid
asriel dreemurr from touhou 93
banished back to middle east
chemistry is applied philosophy
razorback and danger shield is just really bad
```
completely new sentence btw, you just have to give it a lot of data.

## credits
this package is loosely based off the [@jsvine/markovify](https://github.com/jsvine/markovify) library.