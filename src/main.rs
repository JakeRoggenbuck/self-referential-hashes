use md5::{Md5, Digest};
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use std::sync::Arc;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

const BATCH_SIZE: usize = 1000;
const MAX_DIGITS: usize = 16;

fn run(thread_id: i32) {
    loop {
        let start = 16i64.pow(8 as u32 - 1);
        let end = 16i64.pow(9 as u32);

        let total_numbers = end - start;

        let chunks: Vec<_> = (start..end)
            .collect::<Vec<_>>()
            .chunks(BATCH_SIZE)
            .map(|chunk| {
                let chunk_start = *chunk.first().unwrap();
                let chunk_end = *chunk.last().unwrap() + 1;
                (chunk_start, chunk_end)
            })
            .collect();
    }
}

fn main() -> std::io::Result<()> {
    let num_threads = num_cpus::get();

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            std::thread::spawn(move || {
                run(i as i32);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
