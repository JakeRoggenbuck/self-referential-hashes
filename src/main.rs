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
const DIGITS_PER_PHASE: usize = 4;

struct Progress {
    current_phase: AtomicUsize, // Each phase is 4 digits
    found_matches: Mutex<VecDeque<(String, String)>>,
    total_hashes: AtomicUsize,
    phase_complete: AtomicUsize, // Counter for threads that completed current phase
}

impl Progress {
    fn new() -> Self {
        Self {
            current_phase: AtomicUsize::new(0),
            found_matches: Mutex::new(VecDeque::new()),
            total_hashes: AtomicUsize::new(0),
            phase_complete: AtomicUsize::new(0),
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
}

fn run(thread_id: i32, before_string: String, progress: Arc<Progress>) {
    let mut file = File::create(format!("{}_output.txt", thread_id)).unwrap();
<<<<<<< Updated upstream
    let start_time = Instant::now();
    let mut last_report = Instant::now();
    let report_interval = std::time::Duration::from_secs(5);
    let num_threads = num_cpus::get() as i64;
=======
>>>>>>> Stashed changes

    loop {
        let current_phase = progress.current_phase.load(Ordering::SeqCst);
        if current_phase * DIGITS_PER_PHASE >= MAX_DIGITS {
            break;
        }

<<<<<<< Updated upstream
        let phase_start = current_phase * DIGITS_PER_PHASE + 1;
        let phase_end = std::cmp::min((current_phase + 1) * DIGITS_PER_PHASE, MAX_DIGITS);

        println!("Thread {} starting work on digits {}-{}", 
            thread_id, phase_start, phase_end);

        // Process each digit in the current phase
        for digit_length in phase_start..=phase_end {
            let start = if digit_length == 1 { 0 } else { 16i64.pow(digit_length as u32 - 1) };
            let end = 16i64.pow(digit_length as u32);
            let total_numbers = end - start;

            // Split the range among threads
            let chunk_size = (total_numbers + num_threads - 1) / num_threads;
            let thread_start = start + (thread_id as i64 * chunk_size);
            let thread_end = std::cmp::min(thread_start + chunk_size, end);

            println!(
                "Thread {} processing {}-digit numbers from {:x} to {:x} ({} numbers)",
                thread_id, digit_length, thread_start, thread_end - 1, thread_end - thread_start
            );

            // Process this thread's portion of the range
            let num_chunks = ((thread_end - thread_start) as usize + BATCH_SIZE - 1) / BATCH_SIZE;
            
            (0..num_chunks).into_par_iter().for_each(|chunk_idx| {
                let chunk_start = thread_start + (chunk_idx as i64 * BATCH_SIZE as i64);
                let chunk_end = std::cmp::min(chunk_start + BATCH_SIZE as i64, thread_end);
                process_batch(chunk_start, chunk_end, &before_string, &progress, thread_id);
            });

            let elapsed = start_time.elapsed();
            let hashes_per_second = progress.total_hashes.load(Ordering::Relaxed) as f64 / elapsed.as_secs_f64();
            
            println!(
                "Thread {} completed {}-digit numbers ({} hashes) in {:?} ({:.2} hashes/sec)",
                thread_id, digit_length, thread_end - thread_start, elapsed, hashes_per_second
            );
        }

        // Write any found matches to file
        if let Ok(mut matches) = progress.found_matches.lock() {
            while let Some((input, result)) = matches.pop_front() {
                let _ = file.write(format!("MD5({}) = {}\n", input, result).as_bytes());
            }
        }

        // Mark this thread as complete for the current phase
        let completed = progress.phase_complete.fetch_add(1, Ordering::SeqCst) + 1;
        
        // If all threads have completed this phase, move to the next phase
        if completed == num_cpus::get() {
            progress.current_phase.fetch_add(1, Ordering::SeqCst);
            progress.phase_complete.store(0, Ordering::SeqCst);
        } else {
            // Wait for other threads to complete the phase
            while progress.current_phase.load(Ordering::SeqCst) == current_phase {
                std::thread::yield_now();
            }
        }

        // Periodic progress report
        if last_report.elapsed() >= report_interval {
            let total_hashes = progress.total_hashes.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed();
            let hashes_per_second = total_hashes as f64 / elapsed.as_secs_f64();
            println!(
                "Overall progress: {} hashes checked, {:.2} hashes/sec (Phase {})",
                total_hashes, hashes_per_second, current_phase + 1
            );
            last_report = Instant::now();
        }
=======
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

        chunks.par_iter().for_each(|&(chunk_start, chunk_end)| {
            process_batch(chunk_start, chunk_end, &before_string, &progress, thread_id);
        });
>>>>>>> Stashed changes
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
