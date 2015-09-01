use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

fn hex_to_u8(a: u8) -> u8 {
    match a {
        b'0'...b'9' => a - b'0',
        b'a'...b'h' => a - b'a' + 10,
        _           => 0
    }
}

fn vec_u8_from_hex_string(s: &String) -> Vec<u8> {
    s.as_bytes().chunks(2).map(|a| hex_to_u8(a[0]) * 16 + hex_to_u8(a[1])).collect()
}

fn vec_xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity(cmp::min(v1.len(), v2.len()));

    for (a, b) in v1.iter().zip(v2.iter()) {
        v.push(a ^ b);
    }

    v
}

fn main() {
    let input_file = File::open("in.txt").unwrap();
    let input = BufReader::new(input_file);
    let input: Vec<_> = input.lines().filter_map(|res| res.ok())
        .filter(|s| s.len() > 0)
        .map(|s| vec_u8_from_hex_string(&s))
        .collect();
    let input: Vec<_> = input.chunks(2).collect();

    for v in input {
        println!("{:?}", vec_xor(&v[0], &v[1]));
    }
}

