#!/usr/bin/env rust-script
//! dpw@Darryls-iMac.localdomain 2023-02-17 22:39:07
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde = "1"
//! ```

use anyhow::Result;
use std::env;

fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        _ => !(2..n).any(|d| n % d == 0),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args: {:?}", args);

    for arg in args.iter() {
        let n = u64::from_str_radix(arg, 10)?;
        println!("{} is prime? {}", n, is_prime(n));
    }

    Ok(())
}

