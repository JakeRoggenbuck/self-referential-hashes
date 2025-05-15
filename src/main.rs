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
        "Jake wrote this and its MD5 hash happens to start with ",
        "This message was put together by Jake to get an MD5 hash starting with ",
        "Jake’s sentence ends up with an MD5 hash that begins with ",
        "The MD5 hash of what Jake wrote starts with ",
        "Jake came up with this line to get an MD5 hash beginning with ",
        "This line, written by Jake, gives an MD5 hash that starts with ",
        "Here’s a sentence Jake wrote — its MD5 hash starts with ",
        "Jake managed to get the MD5 hash of this message to start with ",
        "This message from Jake has an MD5 hash that starts with ",
        "Jake put this together so the MD5 hash would begin with ",
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
