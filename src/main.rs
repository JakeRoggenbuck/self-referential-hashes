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

struct Progress {
    current_digit: AtomicUsize,
}

impl Progress {
    fn new() -> Self {
        Self {
            current_digit: AtomicUsize::new(1),
        }
    }
}

fn run(thread_id: i32, progress: Arc<Progress>) {
    loop {
        let digit_length = progress.current_digit.fetch_add(1, Ordering::SeqCst);
        if digit_length > MAX_DIGITS {
            break;
        }

        let start = if digit_length == 1 { 0 } else { 16i64.pow(digit_length as u32 - 1) };
        let end = 16i64.pow(digit_length as u32);
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
    let progress = Arc::new(Progress::new());
    let num_threads = num_cpus::get();

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let progress = Arc::clone(&progress);
            std::thread::spawn(move || {
                run(i as i32, progress);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
