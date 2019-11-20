# Rust implementation of JSON ANTLR4 grammar

Experiment of using Rust's traits to implement the JSON ANTLR4 grammar found [here](https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4)

## Basic usage

```
let mut rng = Rng::new();
let mut depth = 0;
let r = Json::generate(&mut rng, &mut depth);
print!("{}", r);
```
