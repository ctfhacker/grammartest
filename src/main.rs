#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

extern crate lazy_static;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

// mod json;
// use json::*;
//
// mod json2;
// use json2::*;

pub mod json3;
use json3::*;

mod rng;
use rng::Rng;

pub const MAX_REPEAT: usize = 64;
pub const MAX_DEPTH: u64 = 64;

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

    /*
    // let mut res = HashSet::new();
    let mut rng = Rng::new();
    let mut res = Vec::new();
    let mut depth = 0;
    let mut start = std::time::Instant::now();

    loop {
        // Generate and attempt to add the testcase to a HashSet,
        // which will implicitly dedup for us
        // res.insert(Json::default().value);
        let r = Json::generate(&mut rng, &mut depth);
        res.push(r);
        if start.elapsed() > std::time::Duration::new(1, 0) {
            print!(
                "Depth: {} -- {:10.3} KB/s\n",
                MAX_DEPTH,
                res.iter().map(|x| x.len()).sum::<usize>() as f64 / 1000.0
            );
            res.clear();
            start = std::time::Instant::now();
        }
    }
    */

    // Multicore support of generating

    // Channel used to send completed work out of the threads to the collector
    let (tx, rx) = channel();

    // Kill signal for the threads to know when to stop
    let die = Arc::new(AtomicBool::new(false));

    // Number threads to start
    let num_cores = 4;

    // Number of results to generate
    let max_results = 1_000_000;

    // Start a thread on each core generating testcases per core
    for _ in 0..num_cores {
        let tx = tx.clone();
        let die = die.clone();
        thread::spawn(move || {
            let mut rng = Rng::new();
            loop {
                let mut depth = 0;
                // Add the test case to the channel to be read
                let _ = tx.send(Json::generate(&mut rng, &mut depth));

                // Check if we should stop the thread
                if die.load(Ordering::Acquire) {
                    break;
                }
            }
        });
    }

    // No need for the tx channel side anymore since we have no more threads
    drop(tx);

    // Add generated results to a HashSet looking for unique test cases
    let mut res = HashSet::new();
    for i in rx.iter() {
        // Do work on the generated testcases
        res.insert(i);

        if res.len() == max_results {
            break;
        }
    }

    // Work is done, tell the worker threads to die
    die.store(true, Ordering::Release);

    print!("{}\n", res.len());
}
