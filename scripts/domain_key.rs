#!/usr/bin/env rust-script
// cargo-deps: rand, chrono

// use std::env;
// use rand::prelude::*;
use chrono::Utc;
use chrono::naive::{NaiveDate, NaiveDateTime};

const ALPHA: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z'
];

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

// fn shard_key() { }

fn to_base62(number: u64) -> String{
    let radix = ALPHA.len() as u64;
    let mut n: u64 = number;
    let mut result: Vec<char> = Vec::with_capacity(30);

    loop {
        let r = (n % radix) as usize ;
        result.push(ALPHA[r]);
        
        n /= radix;
        
        if n <= 0 {
            break;
        }
    }

    result.iter().rev().collect::<String>()
}

fn show(ts: u64, b62: String) {
    println!("ts: {} -> {} : {}", ts, b62, b62.len());
}

fn main() {
    let now = now();
    let fut = NaiveDate::from_ymd(2350, 1, 1).and_hms(0, 0, 0);
    
    let ts: u64 = now.timestamp_micros() as u64;
    let tmax: u64 = fut.timestamp_micros() as u64;
    
    show(ts, to_base62(ts));
    show(tmax, to_base62(tmax));
}

