#![feature(duration_float)]
#![feature(asm)]
#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

extern crate lazy_static;

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

// mod json;
// use json::*;
//
// mod json2;
// use json2::*;

// pub mod json3;
// use json3::*;
//
pub mod json4;
use json4::*;

mod rng;
use rng::Rng;

pub const MAX_REPEAT: usize = 16;
pub const MAX_DEPTH: u64 = 128;

/*
pub trait Generate {
    fn generate(rng: &mut Rng, recursion: &mut u64) -> String;
}
*/

pub trait Generate {
    fn generate(rng: &mut Rng, recursion: &mut u64, buf: &mut Vec<u8>);
}

fn main() {
    // Channel used to send completed work out of the threads to the collector
    let (tx, rx) = channel();

    // Kill signal for the threads to know when to stop
    let die = Arc::new(AtomicBool::new(false));

    // Number threads to start
    let num_cores = 1;

    // Number of results to generate
    let max_results = 10_000_000;

    let mut aux = 0;
    let start = unsafe { core::arch::x86_64::_rdtsc() };
    // let start = std::time::Instant::now();

    // Start a thread on each core generating testcases per core
    for _ in 0..num_cores {
        let tx = tx.clone();
        let die = die.clone();
        thread::spawn(move || {
            let mut rng = Rng::new();
            loop {
                let mut buf = Vec::with_capacity(MAX_DEPTH as usize * 1024);
                let mut depth = 0;
                // Add the test case to the channel to be read
                Json::generate(&mut rng, &mut depth, &mut buf);

                let _ = tx.send(buf);

                // Check if we should stop the thread
                if die.load(Ordering::Acquire) {
                    break;
                }
            }
        });
    }

    // No need for the tx channel side anymore since we have no more threads
    drop(tx);

    let mut generated_bytes = 0;
    let mut counter = 0;
    for generated_input in rx.iter() {
        generated_bytes += generated_input.len();
        counter += 1;

        // let elapsed = start.elapsed();
        let elapsed = unsafe { core::arch::x86_64::_rdtsc() } - start;
        if counter & 0xfffff == 0 {
            print!(
                // "Time: {:10.4?} / {:10} = {:10.4} MiB/s\n",
                "Time: {:10.2?} Mcycle / {:10.2} MB = {:10.4} cycle/byte\n",
                // elapsed.as_secs_f64(),
                elapsed as f64 / 1000. / 1000.,
                generated_bytes as f64 / 1000. / 1000.,
                // generated_bytes as f64 / elapsed as f64 / 1000. / 1000.
                elapsed as f64 / generated_bytes as f64,
            );
        }
    }
}
