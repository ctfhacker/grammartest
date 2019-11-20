#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

use std::sync::atomic::Ordering;

// mod json;
// use json::*;
//
// mod json2;
// use json2::*;

mod json3;
use json3::*;

mod rng;
use rng::Rng;

pub const MAX_REPEAT: usize = 4;
pub const MAX_DEPTH: u64 = 256;

pub trait Generate {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String;
}

fn main() {
    /*
    if true {
        let mut res = HashSet::new();
        loop {
            COUNT.store(0, Ordering::SeqCst);
            if true { 
                if res.len() % 100000 == 0 {
                    print!("Generated: {}\n", res.len());
                }
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
    */

    if true {
        // let mut res = HashSet::new();
        let mut res = Vec::new();
        let mut rng = Rng::new();
        let mut depth = 0;
        let mut start = std::time::Instant::now();

        loop {
            // Generate and attempt to add the testcase to a HashSet,
            // which will implicitly dedup for us
            // res.insert(Json::default().value);
            let r = Json::generate(&mut rng, &mut depth);
            res.push(r);
            if start.elapsed() > std::time::Duration::new(1, 0) {
                print!("Depth: {} -- {:10.3} KB/s\n", MAX_DEPTH, res.iter().map(|x| x.len()).sum::<usize>() as f64 / 1000.0);
                res.clear();
                start = std::time::Instant::now();
            }
        }
    }
}
