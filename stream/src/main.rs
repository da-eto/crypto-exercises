extern crate rand;

use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};
use rand::{ChaChaRng, Rng, SeedableRng};

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

fn encode_char(ch: u8, k: u8) -> u8 {
    match ch {
        b'a'...b'z' => code_to_alpha((alpha_to_code(ch) + alpha_to_code(k)) % 26),
        b'A'...b'Z' => code_to_alpha((alpha_to_code(ch) + alpha_to_code(k)) % 26) - b'a' + b'A',
        _           => ch
    }
}

fn encode(input: &[u8], key: &[u8]) -> String {
    let mut buf = String::with_capacity(input.len());

    for (i, &c) in input.iter().enumerate() {
        buf.push(encode_char(c, key[i % key.len()]) as char);
    }

    buf
}

fn main() {
    let input_file = File::open("in.txt").unwrap();
    let input = BufReader::new(input_file);
    let mut key_file = File::open("in_key.txt").unwrap();
    let mut key = String::new();
    key_file.read_to_string(&mut key).ok();
    let seed: &[_] = &[key.trim().parse::<u32>().ok().unwrap()];
    let mut rng: ChaChaRng = ChaChaRng::new_unseeded();
    let mut output = File::create("out.txt").unwrap();

    for line in input.lines().filter_map(|res| res.ok()) {
        rng.reseed(seed);
        let key: String = rng.gen_ascii_chars().take(line.as_bytes().len()).collect();
        writeln!(&mut output, "{}", encode(line.as_bytes(), key.as_bytes())).unwrap();
    }
}

