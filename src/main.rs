extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::str;

use clap::{App, Arg};

fn main() {
    let m = App::new("wc-rs")
        .version("0.1.0")
        .author("Daniel Toebe <dtoebe@gmail.com>")
        .about("a wc clone in rust")
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("print the word counts"),
        ).arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("print the newline counts"),
        ).arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the bytes counts"),
        ).arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("print the character counts"),
        ).arg(
            Arg::with_name("INPUT")
                .help("file to count from")
                .required(true)
                .takes_value(true),
        ).get_matches();

    let file_path = m.value_of("INPUT").unwrap();

    let mut file = File::open(&file_path).unwrap_or_else(|e| {
        println!("failed to open file: {}", e);
        process::exit(1);
    });

    let mut bytes: Vec<u8> = Vec::new();
    let n = match file.read_to_end(&mut bytes) {
        Ok(n) => n,
        Err(e) => {
            println!("failed to read file: {}", e);
            process::exit(1);
        }
    };

    let mut output = String::new();

    let mut none: bool = false;
    if !m.is_present("words")
        && !m.is_present("lines")
        && !m.is_present("bytes")
        && !m.is_present("chars")
    {
        none = true;
    }

    if m.is_present("words") || none {
        let f = str::from_utf8(&bytes).unwrap_or_else(|e| {
            println!("failed to conver to string: {}", e);
            process::exit(1);
        });

        let n_words = f.split_whitespace().count();
        output.push_str(&format!("{} ", n_words));
    }

    if m.is_present("lines") || none {
        let n_lines = bytes.iter().filter(|b| **b == b'\n').count();
        output.push_str(&format!("{} ", n_lines));
    }

    if m.is_present("bytes") || none {
        output.push_str(&format!("{} ", n));
    }

    if m.is_present("chars") || none {
        let n_chars = bytes.iter().map(|b| *b as char).count();
        output.push_str(&format!("{} ", n_chars));
    }

    println!("{} {}", output, file_path);
}
