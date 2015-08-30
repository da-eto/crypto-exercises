use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

fn is_symbol(ch: u8) -> bool {
    b'!' <= ch && ch <= b'~'
}

fn is_alpha(ch: u8) -> bool {
    (b'a' <= ch && ch <= b'z') || (b'A' <= ch && ch <= b'Z')
}

fn to_ascii(v: &Vec<u8>) -> String {
    let mut s = String::with_capacity(v.len());

    for &ch in v.iter() {
        s.push(if is_symbol(ch) {ch} else {b'_'} as char);
    }

    s
}

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
    let input: Vec<_> = input.lines().filter_map(|res| res.ok()).take(11).map(|s| vec_u8_from_hex_string(&s)).collect();
    let mut key: Vec<u8> = vec![0; 200];
    let mut ws: Vec<u8> = vec![0; 200];

    for i in 0..11 {
        let c_i = &input[i];
        println!("c_{}:", i);
        let c_xor: Vec<_> = input.iter().map(|v| to_ascii(&vec_xor(c_i, &v))).collect();

        for x_j in c_xor.iter() {
            println!("{}", x_j);
        }

        for k in 0..c_i.len() {
            let mut w = 0;

            for x_j in c_xor.iter() {
                if k < x_j.len() && is_alpha(x_j.as_bytes()[k]) {
                    w += 1;
                }
            }

            if w > ws[k] {
                key[k] = c_i[k] ^ b' ';
                ws[k] = w;
            }
        }

        println!("key: {:?}\n", key);
    }

    for i in 0..11 {
        println!("msg {}:\t{}\n", i, to_ascii(&vec_xor(&input[i], &key)));
    }
}

