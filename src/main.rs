#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

mod json;
use json::*;

pub const MAX_REPEAT: usize = 1000;
pub const MAX_STEPS: u64 = 256000;

fn main() {
    let mut res = HashSet::new();
    loop {
        if res.len() % 100000 == 0 {
            print!("Generated: {}\n", res.len());
        }
        res.insert(Json::default().value);
        if res.len() >= 1000000 {
            break 
        }
    }
    print!("Generated: {}\n", res.len());
}
