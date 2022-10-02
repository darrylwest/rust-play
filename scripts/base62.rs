#!/usr/bin/env rust-script
// cargo-deps: rand, chrono

// use std::env;
// use rand::{Rng, thread_rng};
// use chrono::Utc;
// use chrono::naive::{NaiveDate, NaiveDateTime};

pub const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z'
];

pub const MAX_64: u64 = 3521614000000;
pub const MIN_64: u64 = 56810000000;

pub fn to_base62(number: u64) -> String{
    let radix = ALPHA.len() as u64;
    let mut n: u64 = number;
    let mut result: Vec<char> = Vec::with_capacity(30);

    loop {
        let idx = (n % radix) as usize ;
        result.push(ALPHA[idx]);
        
        n /= radix;
        
        if n == 0 {
            break;
        }
    }

    result.iter().rev().collect::<String>()
}

fn decode_digit(digit: u8) -> u8 {
    const ZERO: u8 = 48;
    const NINE: u8 = 57;
    const BIG_A: u8 = 65;
    const BIG_Z: u8 = 65 + 26;
    const LITTLE_A: u8 = 97;
    const LITTLE_Z: u8 = 97 + 26;

    match digit {
        ZERO..=NINE => digit - ZERO,
        BIG_A..=BIG_Z => digit - BIG_A + 10,
        LITTLE_A..=LITTLE_Z => digit - LITTLE_A + 36,
        _ => panic!("out of range"),
    }
}

fn from_base62(b62: &String) -> u64 {
    let radix = 62_u64;
    let mut result = 0_u64;
    let mut p = 0_u32;

    for ch in b62.chars().rev() {
        let n = decode_digit(ch as u8) as u64;

        assert!(n < radix);

        let q = radix.pow(p);
        assert!(q > 0);
        result += n * q;

        println!("ch: {} n: {} result: {} q: {} p: {}", ch, n, result, q, p);

        p += 1;
    }

    result
}

fn test(x: u64) {
    let b62 = to_base62(x);
    let result = from_base62(&b62);
    println!("{} -> {} -> {}", x, b62, result);
    assert_eq!(result, x);

    println!("{} is ok\n", x);
}

fn main() {
    let values = [32_u64, 62_u64, 1_000_u64, 1_000_000_u64, 19501103_u64, 20000101_u64, 10_000_000_u64, ];

    for v in values {
        test(v);
    }

    /*
    for v in 32..1_000_000_u64 {
        test(v);
    }
    */
}

