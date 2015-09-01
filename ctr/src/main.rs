extern crate crypto;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crypto::aessafe::{AesSafe128Decryptor, AesSafe128Encryptor};
use crypto::symmetriccipher::{BlockDecryptor, BlockEncryptor};

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

fn u8_slice_inc(v: &mut [u8]) {
    let mut inc = 1;

    for i in (0..v.len()).rev() {
        if v[i] == 0xff && inc == 1 {
            v[i] = 0;
            inc = 1;
        } else {
            v[i] += inc;
            inc = 0;
        }
    }
}

fn ctr_decrypt(key: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity(msg.len());
    let (iv, msg) = msg.split_at(16);
    // TODO: find better way to copy arrays
    let mut prev = &mut [0; 16];

    for i in 0..16 {
        prev[i] = iv[i];
    }

    let mut m_i = &mut [0; 16];
    let decryptor = AesSafe128Encryptor::new(&key[..]);

    for c_i in msg.chunks(16) {
        decryptor.encrypt_block(prev, &mut m_i[..]);

        for i in 0..c_i.len() {
            v.push(c_i[i] ^ m_i[i]);
        }

        u8_slice_inc(prev);
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
        println!("key: {:?}", v[0]);
        println!("cpt: {:?}", v[1]);

        let msg = ctr_decrypt(&v[0], &v[1]);
        println!("msg: {:?}", msg);

        let pt = String::from_utf8(msg).unwrap();
        println!("pt: {:?}", pt);

        println!("");
    }
}

