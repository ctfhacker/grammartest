#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

mod json;
use json::*;

pub const MAX_REPEAT: usize = 1024;
pub const MAX_STEPS: u64 = 256000;

fn main() {
    let mut res = HashSet::new();
    loop {
        // Print status
        if res.len() % 100000 == 0 {
            print!("Generated: {}\n", res.len());
        }

        // Generate and attempt to add the testcase to a HashSet,
        // which will implicitly dedup for us
        res.insert(Json::default().value);

        // If we hit our maximum of 1M test cases, break
        if res.len() >= 1000000 {
            break 
        }
    }
    print!("Generated: {}\n", res.len());
}
