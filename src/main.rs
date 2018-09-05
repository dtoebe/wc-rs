use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage here...");
        process::exit(1);
    }

    let file_path = args[1].clone();

    let mut file = File::open(&file_path).unwrap_or_else(|e| {
        println!("failed to open file: {}", e);
        process::exit(1);
    });

    // let mut buf = BufReader::new(file);
    // let byte_buf = buf.by_ref();
    // let byte_count = byte_buf.bytes().count();
    // let line_count = byte_buf.lines().count();

    let mut bytes: Vec<u8> = Vec::new();
    let n_bytes = match file.read_to_end(&mut bytes) {
        Ok(n) => n,
        Err(e) => {
            println!("failed to read file: {}", e);
            process::exit(1);
        }
    };

    let n_chars = bytes.iter().map(|b| *b as char).count();
    let n_lines = bytes.iter().filter(|b| **b == b'\n').count();
    let f = str::from_utf8(&bytes).unwrap_or_else(|e| {
        println!("failed to conver to string: {}", e);
        process::exit(1);
    });

    let n_words = f.split_whitespace().count();

    println!("{} {}", n_bytes, file_path);
    println!("{} {}", n_words, file_path);
    println!("{} {}", n_chars, file_path);
    println!("{} {}", n_lines, file_path);
}
