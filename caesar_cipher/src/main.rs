use std::fs::File;
use std::io::{BufReader, BufRead, Write};

fn encode_char(ch: u8) -> u8 {
    let lower = b'a' <= ch && ch <= b'z';
    let upper = b'A' <= ch && ch <= b'Z';

    if lower || upper {
        let c = ch + 3;

        if (lower && c > b'z') || (upper && c > b'Z') {
            c - 26
        } else {
            c
        }
    } else {
        ch
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

