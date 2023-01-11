#!/usr/bin/env rust-script
// cargo-deps: rand

// use std::any::{Any, TypeId};
use std::collections::HashMap;
use rand::prelude::*;
use std::char::from_digit;

fn create_key() -> String {
    let mut rng = rand::thread_rng();
    let mut results: Vec<char> = vec![];

    for _ in 0..12 {
        let n = rng.gen_range(0..36);
        results.push(from_digit(n as u32, 36).unwrap());
    }

    results.into_iter().rev().collect()
}


fn main() {
    let mut hmap: HashMap<String, String> = HashMap::new();

    let key = create_key();

    println!("key: {}", key);

    hmap.insert(key, "my value".to_string());

    println!("hmap: {:?}", hmap);

}
