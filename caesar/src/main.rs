use std::fs::File;
use std::io::{BufReader, BufRead, Write};

fn alpha_to_code(ch: u8) -> u8 {
    match ch {
        b'a'...b'z' => ch - b'a',
        b'A'...b'Z' => ch - b'A',
        _           => ch
    }
}

fn code_to_alpha(c: u8) -> u8 {
    c + b'a'
}

fn encode_char(ch: u8) -> u8 {
    match ch {
        b'a'...b'z' => code_to_alpha((alpha_to_code(ch) + 3) % 26),
        b'A'...b'Z' => code_to_alpha((alpha_to_code(ch) + 3) % 26) - b'a' + b'A',
        _           => ch
    }
}

fn encode(input: &[u8]) -> String {
    let mut buf = String::with_capacity(input.len());

    for &c in input {
        buf.push(encode_char(c) as char);
    }

    buf
}

fn main() {
    let input_file = File::open("in.txt").unwrap();
    let input = BufReader::new(input_file);
    let mut output = File::create("out.txt").unwrap();

    for line in input.lines().filter_map(|res| res.ok()) {
        writeln!(&mut output, "{}", encode(line.as_bytes())).unwrap();
    }
}

