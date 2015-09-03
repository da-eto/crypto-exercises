extern crate crypto;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crypto::aessafe::AesSafe128Encryptor;
use crypto::symmetriccipher::BlockEncryptor;

fn hex_to_u8(a: u8) -> u8 {
    match a {
        b'0'...b'9' => a - b'0',
        b'a'...b'h' => a - b'a' + 10,
        _           => 0
    }
}

fn u8_to_hex(a: u8) -> u8 {
    match a {
        0...9   => a + b'0',
        10...15 => a + b'a' - 10,
        _       => 0
    }
}

fn vec_u8_from_hex_string(s: &String) -> Vec<u8> {
    s.as_bytes().chunks(2).map(|a| hex_to_u8(a[0]) * 16 + hex_to_u8(a[1])).collect()
}

fn vec_u8_to_hex_string(v: &Vec<u8>) -> String {
    v.iter().map(|u| [u8_to_hex(u / 16), u8_to_hex(u % 16)])
        .fold(String::from(""), |mut acc, item| {acc.push(item[0] as char); acc.push(item[1] as char); acc})
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

fn ctr_crypt(key: &[u8], iv: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(msg.len());
    // TODO: find better way to copy arrays
    let mut prev = &mut [0; 16];

    for i in 0..16 {
        prev[i] = iv[i];
    }

    let mut m_i = &mut [0; 16];
    let encryptor = AesSafe128Encryptor::new(key);

    for c_i in msg.chunks(16) {
        encryptor.encrypt_block(prev, &mut m_i[..]);

        for i in 0..c_i.len() {
            v.push(c_i[i] ^ m_i[i]);
        }

        u8_slice_inc(prev);
    }

    v
}

fn ctr_decrypt(key: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let (iv, msg) = msg.split_at(16);
    ctr_crypt(&key[..], iv, &msg[..])
}

fn ctr_encrypt(key: &Vec<u8>, iv: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity(iv.len() + msg.len());
    v.extend(iv.iter().cloned());
    v.extend(ctr_crypt(&key[..], &iv[..], &msg[..]));
    v
}

fn main() {
    // decode
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

    // encode
    let input_file = File::open("in_clear.txt").unwrap();
    let input = BufReader::new(input_file);
    let input: Vec<_> = input.lines().filter_map(|res| res.ok())
        .filter(|s| s.len() > 0)
        .collect();
    let input: Vec<_> = input.chunks(3).collect();

    for v in input {
        let key = vec_u8_from_hex_string(&v[0]);
        let iv = vec_u8_from_hex_string(&v[1]);
        let pt = &v[2];

        println!("key: {:?}", key);
        println!("IV: {:?}", iv);
        println!("pt: {:?}", pt);

        let ct = ctr_encrypt(&key, &iv, &pt.as_bytes().iter().cloned().collect());
        println!("msg: {:?}", ct);

        let hex = vec_u8_to_hex_string(&ct);
        println!("ct: {:?}", hex);

        println!("");
    }
}

