#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

// mod json;
// use json::*;
//
mod json2;
use json2::*;

mod rng;
use rng::Rng;

pub const MAX_REPEAT: usize = 4;
pub const MAX_STEPS: u64 = 256000;

pub trait Generate {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String;
}

fn main() {
    let mut res = HashSet::new();
    let mut rng = Rng::new();
    let mut recursion = 0;
    loop {
        recursion = 0;
        // Print status
        if true { 
            if res.len() % 100000 == 0 {
                print!("Generated: {}\n", res.len());
            }
        }

        // Generate and attempt to add the testcase to a HashSet,
        // which will implicitly dedup for us
        // res.insert(Json::default().value);
        let r = Json::generate(&mut rng, &mut recursion);
        print!("{}\n", r);
        res.insert(r);

        // If we hit our maximum of 1M test cases, break
        if res.len() >= 1000000 {
            break 
        }
    }

    print!("Generated: {}\n", res.len());
}
