use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn run(number: i32, before_string: String) {
    let mut file = File::create(format!("{}_output.txt", number)).unwrap();
    let f: i64 = 10000000000000;
    for i in 0..f {
        let mut hasher = Md5::new();
        let a = format!("{}{:x}", before_string, i);

        hasher.update(a.clone());
        let result = hasher.finalize();

        let b = format!("{:x}", i);
        let c: Vec<char> = format!("{:x}", result).chars().collect();

        let mut i = 0;
        let mut found = true;
        for x in b.chars() {
            if c[i] != x {
                found = false;
            }
            i += 1;
        }
        if found {
            println!("MD5({}) = {:x}", a, result);
            let d: &str = &format!("MD5({}) = {:x}\n", a, result);
            file.write(d.as_bytes());
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut handles = vec![];

    let strings = vec![
        "Jake's MD5 hash: ",
        "Jake's MD5 hash = ",
        "Jake's hash: ",
        "Jake's hash = ",
        "Silly hash = ",
        "Silly hash: ",
        "Silly MD5 hash = ",
        "Silly MD5 hash: ",
        "silly MD5 hash = ",
        "silly MD5 hash: ",
    ];

    for i in 0..9 {
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
