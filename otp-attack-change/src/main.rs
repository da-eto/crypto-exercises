use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};

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

fn hex_to_u8(a: u8) -> u8 {
    match a {
        b'0'...b'9' => a - b'0',
        b'a'...b'h' => a - b'a' + 10,
        _           => 0
    }
}

fn u8_to_hex(h: u8) -> u8 {
    match h {
        0...9   => h + b'0',
        10...15 => h - 10 + b'a',
        _       => 0
    }
}

fn vec_u8_from_hex_string(s: &String) -> Vec<u8> {
    s.as_bytes().chunks(2).map(|a| hex_to_u8(a[0]) * 16 + hex_to_u8(a[1])).collect()
}

fn hex_string_from_vec(v: &Vec<u8>) -> String {
    let mut s = String::with_capacity(v.len() * 2);

    for (i, &c) in v.iter().enumerate() {
        s.push(u8_to_hex(c / 16) as char);
        s.push(u8_to_hex(c % 16) as char);
    }

    s
}

fn main() {
    let input_file = File::open("in.txt").unwrap();
    let input = BufReader::new(input_file);
    let input: Vec<_> = input.lines().filter_map(|res| res.ok()).take(3).collect();
    let in1 = &input[0].as_bytes();
    let in2 = &input[1].as_bytes();
    let cipher = vec_u8_from_hex_string(&input[2]);
    let mut res: Vec<u8> = Vec::with_capacity(in1.len());

    for (i, &c) in cipher.iter().enumerate() {
        res.push(c ^ in1[i] ^ in2[i]);
    }

    println!("{:?}\n{:?}\n{:?}\n{:?}", in1, in2, cipher, hex_string_from_vec(&res));
}

