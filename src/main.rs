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
    found_matches: Mutex<VecDeque<(String, String)>>,
    total_hashes: AtomicUsize,
}

impl Progress {
    fn new() -> Self {
        Self {
            current_digit: AtomicUsize::new(1),
            found_matches: Mutex::new(VecDeque::new()),
            total_hashes: AtomicUsize::new(0),
        }
    }
}

fn process_batch(
    start: i64,
    end: i64,
    before_string: &str,
    progress: &Progress,
    thread_id: i32,
) {
    let mut batch_results = Vec::new();
    
    for i in start..end {
        let input = format!("{}{:x}", before_string, i);
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        
        let hex_input = format!("{:x}", i);
        let hex_result = format!("{:x}", result);
        
        if hex_result.starts_with(&hex_input) {
            batch_results.push((input, hex_result));
        }
    }
    
    progress.total_hashes.fetch_add((end - start) as usize, Ordering::Relaxed);
}

fn run(thread_id: i32, before_string: String, progress: Arc<Progress>) {
    loop {
        let digit_length = progress.current_digit.fetch_add(1, Ordering::SeqCst);
        if digit_length > MAX_DIGITS {
            break;
        }

        let start = if digit_length == 1 { 0 } else { 16i64.pow(digit_length as u32 - 1) };
        let end = 16i64.pow(digit_length as u32);
        let total_numbers = end - start;

        // Process in batches for better cache utilization
        let chunks: Vec<_> = (start..end)
            .collect::<Vec<_>>()
            .chunks(BATCH_SIZE)
            .map(|chunk| {
                let chunk_start = *chunk.first().unwrap();
                let chunk_end = *chunk.last().unwrap() + 1;
                (chunk_start, chunk_end)
            })
            .collect();

        chunks.par_iter().for_each(|&(chunk_start, chunk_end)| {
            process_batch(chunk_start, chunk_end, &before_string, &progress, thread_id);
        });
    }
}

fn main() -> std::io::Result<()> {
    let progress = Arc::new(Progress::new());
    let target_string = "Here's a sentence Jake wrote and its MD5 hash starts with ".to_string();
    let num_threads = num_cpus::get();

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let progress = Arc::clone(&progress);
            let target_string = target_string.clone();
            std::thread::spawn(move || {
                run(i as i32, target_string, progress);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
