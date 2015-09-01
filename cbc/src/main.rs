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

fn cbc_decrypt(key: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity(msg.len());
    let (iv, msg) = msg.split_at(16);
    // TODO: find better way to copy arrays
    let mut prev = &mut [0; 16];

    for i in 0..16 {
        prev[i] = iv[i];
    }

    let mut m_i = &mut [0; 16];
    let decryptor = AesSafe128Decryptor::new(&key[..]);

    for c_i in msg.chunks(16) {
        decryptor.decrypt_block(c_i, &mut m_i[..]);

        for i in 0..m_i.len() {
            v.push(m_i[i] ^ prev[i]);
        }

        for i in 0..prev.len() {
            prev[i] = c_i[i];
        }
    }

    let pad = v.pop().unwrap();

    for _ in 1..pad {
        v.pop();
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

        let msg = cbc_decrypt(&v[0], &v[1]);
        println!("msg: {:?}", msg);

        let pt = String::from_utf8(msg).unwrap();
        println!("pt: {:?}", pt);

        println!("");
    }
}

