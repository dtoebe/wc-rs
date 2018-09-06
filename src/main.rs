extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::Read;
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
                .takes_value(true),
        ).get_matches();

    let mut file_path = String::new();
    let bytes = if m.is_present("INPUT") {
        file_path.push_str(m.value_of("INPUT").unwrap());
        file_to_bytes(file_path.clone()).unwrap()
    } else {
        input_to_bytes()
    };
    let n = bytes.len();

    if n <= 0 {
        eprintln!("no input found please provide an input");
        process::exit(1);
    }

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
            eprintln!("failed to convert to string: {}", e);
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

fn file_to_bytes(file_path: String) -> Result<Vec<u8>, Box<Error>> {
    let mut file = File::open(&file_path)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

fn input_to_bytes() -> Vec<u8> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    while let Ok(n_bytes) = stdin.read_to_string(&mut input) {
        if n_bytes == 0 {
            break;
        }
        input.push_str(&line);
        line.clear();
    }
    input.as_bytes().to_vec()
}
