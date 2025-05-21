use md5::{Digest, Md5};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

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

fn process_batch(start: i64, end: i64, before_string: &str, progress: &Progress, thread_id: i32) {
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

    progress
        .total_hashes
        .fetch_add((end - start) as usize, Ordering::Relaxed);

    if !batch_results.is_empty() {
        let mut matches = progress.found_matches.lock().unwrap();
        for (input, result) in batch_results {
            let input_clone = input.clone();
            let result_clone = result.clone();
            matches.push_back((input, result));
            println!(
                "Thread {} found match: MD5({}) = {}",
                thread_id, input_clone, result_clone
            );
        }
    }
}

fn run(thread_id: i32, before_string: String, progress: Arc<Progress>) {
    let mut file = File::create(format!("{}_output.txt", thread_id)).unwrap();
    let start_time = Instant::now();
    let mut last_report = Instant::now();
    let report_interval = std::time::Duration::from_secs(5);

    loop {
        let digit_length = progress.current_digit.fetch_add(1, Ordering::SeqCst);
        if digit_length > MAX_DIGITS {
            break;
        }

        println!(
            "Thread {} starting work on {}-digit numbers",
            thread_id, digit_length
        );

        let start = if digit_length == 1 {
            0
        } else {
            16i64.pow(digit_length as u32 - 1)
        };
        let end = 16i64.pow(digit_length as u32);
        let total_numbers = end - start;

        // Process ranges directly without collecting them
        let num_chunks = ((end - start) as usize + BATCH_SIZE - 1) / BATCH_SIZE;

        (0..num_chunks).into_par_iter().for_each(|chunk_idx| {
            let chunk_start = start + (chunk_idx as i64 * BATCH_SIZE as i64);
            let chunk_end = std::cmp::min(chunk_start + BATCH_SIZE as i64, end);
            process_batch(chunk_start, chunk_end, &before_string, &progress, thread_id);
        });

        let elapsed = start_time.elapsed();
        let hashes_per_second =
            progress.total_hashes.load(Ordering::Relaxed) as f64 / elapsed.as_secs_f64();

        println!(
            "Thread {} completed {}-digit numbers ({} hashes) in {:?} ({:.2} hashes/sec)",
            thread_id, digit_length, total_numbers, elapsed, hashes_per_second
        );

        // Write any found matches to file
        if let Ok(mut matches) = progress.found_matches.lock() {
            while let Some((input, result)) = matches.pop_front() {
                let _ = file.write(format!("MD5({}) = {}\n", input, result).as_bytes());
            }
        }

        // Periodic progress report
        if last_report.elapsed() >= report_interval {
            let total_hashes = progress.total_hashes.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed();
            let hashes_per_second = total_hashes as f64 / elapsed.as_secs_f64();
            println!(
                "Overall progress: {} hashes checked, {:.2} hashes/sec",
                total_hashes, hashes_per_second
            );
            last_report = Instant::now();
        }
    }
}

fn main() -> std::io::Result<()> {
    let progress = Arc::new(Progress::new());
    let target_string = "Here's a sentence Jake wrote and its MD5 hash starts with ".to_string();
    let num_threads = num_cpus::get();

    println!("Starting with {} threads", num_threads);

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
