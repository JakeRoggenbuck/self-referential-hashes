use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn run(number: i32, before_string: String) {
    let mut file = File::create(format!("{}_output.txt", number)).unwrap();
    for i in 0..i64::MAX {
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
            println!(
                "MD5({}) = {:x}, and {} hashes checked for thread {}",
                a, result, i, number
            );
            let d: &str = &format!("MD5({}) = {:x}\n", a, result);
            let _ = file.write(d.as_bytes());
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut handles = vec![];

    let strings = vec![
        "This text made by Jake has an MD5 hash starting with ",
        "Jake crafted this message so its MD5 hash starts with ",
        "This sentence, written by Jake, has an MD5 hash beginning with ",
        "The MD5 hash of this Jake-authored text begins with ",
        "Jake composed this string to produce an MD5 hash that starts with ",
        "This string, designed by Jake, yields an MD5 hash starting with ",
        "Written by Jake, this message’s MD5 hash begins with ",
        "Jake’s text was engineered to start its MD5 hash with ",
        "This is a message from Jake whose MD5 hash starts with ",
        "Jake made this message so that its MD5 hash would begin with ",
    ];

    for i in 0..7 {
        let c = strings[i as usize].to_string();

        let handle = thread::spawn(move || {
            run(i, c);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
