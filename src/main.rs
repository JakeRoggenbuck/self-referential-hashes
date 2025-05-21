use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

fn run(number: i32, before_string: String, current_digit: &AtomicUsize) {
    let mut file = File::create(format!("{}_output.txt", number)).unwrap();

    loop {
        // Get the next digit length to process
        let digit_length = current_digit.fetch_add(1, Ordering::SeqCst);
        if digit_length > 16 { // Limit to reasonable number of digits
            break;
        }

        let start_time = Instant::now();
        println!("Thread {} starting work on {}-digit numbers", number, digit_length);
        
        // Calculate range for this digit length
        let start = if digit_length == 1 { 0 } else { 16i64.pow(digit_length as u32 - 1) };
        let end = 16i64.pow(digit_length as u32);
        let total_numbers = end - start;

        for i in start..end {
            let mut hasher = Md5::new();
            let a = format!("{}{:x}", before_string, i);

            hasher.update(a.clone());
            let result = hasher.finalize();

            let b = format!("{:x}", i);
            let c: Vec<char> = format!("{:x}", result).chars().collect();

            let mut j = 0;
            let mut found = true;

            for x in b.chars() {
                if c[j] != x {
                    found = false;
                }
                j += 1;
            }

            if found {
                let elapsed = start_time.elapsed();
                println!(
                    "MD5({}) = {:x}, and {} hashes checked for thread {} in {:?}",
                    a, result, i, number, elapsed
                );
                let d: &str = &format!("MD5({}) = {:x}\n", a, result);
                let _ = file.write(d.as_bytes());
            }
        }

        let elapsed = start_time.elapsed();
        println!(
            "Thread {} completed {}-digit numbers ({} hashes) in {:?}",
            number, digit_length, total_numbers, elapsed
        );
    }
}

fn main() -> std::io::Result<()> {
    let mut handles = vec![];
    let current_digit = AtomicUsize::new(1); // Start with 1-digit numbers

    let strings = vec![
        "Jake wrote this and its MD5 hash happens to start with ",
        "This message was put together by Jake to get an MD5 hash starting with ",
        "Jake's sentence ends up with an MD5 hash that begins with ",
        "The MD5 hash of what Jake wrote starts with ",
        "Jake came up with this line to get an MD5 hash beginning with ",
        "This line, written by Jake, gives an MD5 hash that starts with ",
        "Here's a sentence Jake wrote — its MD5 hash starts with ",
        "Jake managed to get the MD5 hash of this message to start with ",
        "This message from Jake has an MD5 hash that starts with ",
        "Jake put this together so the MD5 hash would begin with ",
    ];

    for i in 0..7 {
        let c = strings[i as usize].to_string();
        let current_digit = &current_digit;

        let handle = thread::spawn(move || {
            run(i, c, current_digit);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
