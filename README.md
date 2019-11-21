# Rust implementation of JSON ANTLR4 grammar

Experiment of using Rust's traits to implement the JSON ANTLR4 grammar found [here](https://github.com/antlr/grammars-v4/blob/master/json/JSON.g4)

Toying around with ideas of how to implement [uniform distribution](https://github.com/ctfhacker/grammartest/blob/master/src/json3.rs#L113) of right hand results vs [probability based decisions](https://github.com/ctfhacker/grammartest/blob/master/src/json3.rs#L218).

## Basic usage

```
let mut rng = Rng::new();
let mut depth = 0;
let r = Json::generate(&mut rng, &mut depth);
print!("{}", r);
```
