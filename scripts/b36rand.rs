#!/usr/bin/env rust-script
// cargo-deps: rand

use rand::prelude::*;
use std::env;

fn base36_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut results = vec![0u8; 0];

    for _ in 0..size {
        results.push(rng.gen_range(0..36));
    }

    results
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        32
    };

    let bytes = base36_bytes(count);

    let mut results: Vec<char> = vec![];

    for byte in bytes {
        results.push(std::char::from_digit(byte as u32, 36).unwrap())
    }

    let s: String = results.into_iter().rev().collect();

    println!("{}", s);
}
