//! BNF Grammar using Rust Default trait
//!
//! http://www.cs.utsa.edu/~wagner/CS3723/grammar/examples.html
//!
//! A grammar for simple sentences. Things in this language are:
//!     THE MAN BITES A DOG
//!     A DOG PETS A DOG
//! Things not in this language are:
//!     MAN BITES DOG
//! Here is the grammar:
//!     <sentence> ::= <subject> <predicate>
//!     <subject> ::= <article> <noun>
//!     <predicate> ::= <verb> <direct-object>
//!     <direct-object> ::= <article> <noun>
//!     <article> ::= THE | A
//!     <noun> ::= MAN | DOG
//!     <verb> ::= BITES | PETS

#![feature(asm)]
#![feature(exclusive_range_pattern)]
use core::sync::atomic::{Ordering, AtomicU64};

#[macro_use]
extern crate lazy_static;

fn rdrand() -> u64 {
    let res: u64;
    unsafe { asm!("rdrand $0" : "=r"(res)); }
    res
}

lazy_static! {
    static ref SEED: AtomicU64 = AtomicU64::new(rdrand());
}

/// Rng seeded with rdrand that is generated using Lehmer64
fn rand() -> usize {
    let mut val = SEED.load(Ordering::SeqCst);
    val ^= val << 13;
    val ^= val >> 17;
    val ^= val << 43;
    SEED.store(val, Ordering::SeqCst);
    val as usize
}

#[derive(Default, Debug)]
struct Sentence {
    subject: Subject,
    predicate: Predicate

}
impl ToString for Sentence {
    fn to_string(&self) -> String {
        self.subject.to_string() + &" " + &self.predicate.to_string()
    }
}


#[derive(Debug, Default)]
struct Subject { 
    article: Article,
    noun: Noun

}
impl ToString for Subject {
    fn to_string(&self) -> String {
        // self.article.to_string() + &" " + &self.noun.to_string()
        [self.article.value, self.noun.value].join(" ")
    }
}

#[derive(Debug, Default)]
struct Predicate { 
    verb: Verb,
    direct_object: DirectObject
}
impl ToString for Predicate {
    fn to_string(&self) -> String {
        [self.verb.value, self.direct_object.to_string().as_str()].join(" ")
    }
}

#[derive(Debug, Default)]
struct DirectObject { 
    article: Article,
    noun: Noun
}
impl ToString for DirectObject {
    fn to_string(&self) -> String {
        [self.article.value, self.noun.value].join(" ")
    }
}

#[derive(Debug)]
struct Article { pub value: &'static str }
impl Default for Article {
    fn default() -> Article {
        // Probability result
        let value = match rand() % 100 {
            0..10 => "THE",
            _ => "A"
        };
        Article { value }
    }
}

macro_rules! symbol {
    ($sym:ident, $matches:expr) => {
        #[derive(Debug)]
        struct $sym { pub value: &'static str }
        impl Default for $sym {
            fn default() -> $sym {
                // Average weights between options
                let values = $matches;
                let value = values[rand() % values.len()];
                $sym { value }
            }
        }
    }
}

symbol!(Noun, ["MAN", "DOG"]);
symbol!(Verb, ["BITES", "PETS"]);

fn main() {
    for _ in 0..100 {
        print!("{:?}\n", Sentence::default().to_string());
    }
}
