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

fn cbc_decrypt(key: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let decryptor = AesSafe128Decryptor::new(&key[..]);
    let mut v = Vec::with_capacity(msg.len());
    let (iv, msg) = msg.split_at(16);
    // TODO: find better way to copy arrays
    let mut prev = &mut [0; 16];

    for i in 0..16 {
        prev[i] = iv[i];
    }

    let mut m_i = &mut [0; 16];

    for c_i in msg.chunks(16) {
        decryptor.decrypt_block(c_i, &mut m_i[..]);

        for i in 0..m_i.len() {
            m_i[i] ^= prev[i];
        }
        
        v.extend(m_i.iter().cloned());

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

fn cbc_encrypt(key: &Vec<u8>, iv: &Vec<u8>, msg: &Vec<u8>) -> Vec<u8> {
    let encryptor = AesSafe128Encryptor::new(&key[..]);
    // TODO: find better way to copy arrays
    let mut prev = &mut [0; 16];

    for i in 0..16 {
        prev[i] = iv[i];
    }
    
    let mut v = Vec::with_capacity(msg.len());
    v.extend(iv.iter().cloned());
    
    let mut m_i = &mut [0; 16];
    let mut c_i = &mut [0; 16];
    let msg_padded = {
        let pad = 16 - (msg.len() % 16) as u8;
        let mut t = Vec::with_capacity(msg.len() + pad as usize);
        t.extend(msg.iter().cloned());
        
        for _ in 0..pad {
            t.push(pad);                
        }
        
        t
    };

    for m in msg_padded.chunks(16) {
        for i in 0..m.len() {
            m_i[i] = m[i] ^ prev[i];            
        }
        
        encryptor.encrypt_block(m_i, &mut c_i[..]);
        v.extend(c_i.iter().cloned());

        for i in 0..prev.len() {
            prev[i] = c_i[i];
        }
    }

    v
}

fn main() {
    // decrypt
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

    // encrypt
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

        let ct = cbc_encrypt(&key, &iv, &pt.as_bytes().iter().cloned().collect());
        println!("msg: {:?}", ct);

        let hex = vec_u8_to_hex_string(&ct);
        println!("ct: {:?}", hex);

        println!("");
    }
}
