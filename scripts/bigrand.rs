#!/usr/bin/env rust-script
// cargo-deps: num-bigint, rand

use num_bigint::BigUint;
use rand::prelude::*;

fn main() {
    let count = 8;
    let mut rng = rand::thread_rng();

    let mut bytes: Vec<u32> = vec![];
    for _ in 0..count {
        bytes.push(rng.gen());
    }

    println!("{:?}", bytes);

    let r = BigUint::new(bytes);

    println!("{:?}", r);
    println!("{:?}", r.to_str_radix(36));
}
