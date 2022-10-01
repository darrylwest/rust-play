#!/usr/bin/env rust-script
// cargo-deps: rand, chrono

// use std::env;
use rand::{Rng, thread_rng};
use chrono::Utc;
use chrono::naive::{NaiveDate, NaiveDateTime};

const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z'
];

const MAX_64: u64 = 3521614000000;
const MIN_64: u64 = 56810000000;
// delta = 13_316_600_000_000_000 

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

fn generate_random() -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(MIN_64..MAX_64)
}

// fn shard_key() { }

fn to_base62(number: u64) -> String{
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

fn show(label: &str, ts: u64, b62: String) {
    println!("{:18}:{} -> {} : {}", label, ts, b62, b62.len());
}

fn test_micros(dt: NaiveDateTime) -> u64 {
    dt.timestamp_micros() as u64
}

fn test_nanos(dt: NaiveDateTime) -> u64 {
    dt.timestamp_nanos() as u64
}

fn main() {
    let now = now();
    let fut = NaiveDate::from_ymd(2350, 1, 1).and_hms(0, 0, 0);
    
    let ts: u64 = test_micros(now);
    let tmax: u64 = test_micros(fut);
    
    show("micros now", ts, to_base62(ts));
    show("micros max date", tmax, to_base62(tmax));

    let ts: u64 = test_nanos(now);
    let tmax: u64 = test_nanos(fut);
    
    show("nanos now", ts, to_base62(ts));
    show("nanos max date", tmax, to_base62(tmax));

    show("min u64", MIN_64, to_base62(MIN_64));
    show("max u64", MAX_64, to_base62(MAX_64));

    println!("\nrandom generation between {} and {} always generates 7 base62 chars", MIN_64, MAX_64);
    println!("max - min range: {}", MAX_64 - MIN_64);

    let n = generate_random();
    show("random", n, to_base62(n));
}

