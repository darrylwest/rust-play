#!/usr/bin/env rust-script
// cargo-deps: rand, chrono

// use std::env;
// use rand::prelude::*;
use chrono::Utc;
use chrono::naive::{NaiveDate, NaiveDateTime};

const ALPHA: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
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

// fn to_base62() { }


fn main() {
    let now = now();
    let fut = NaiveDate::from_ymd(2300, 1, 1).and_hms(0, 0, 0);
    
    let ts: u64 = now.timestamp_micros() as u64;
    let tmax: u64 = fut.timestamp_micros() as u64;
    
    let s = ALPHA.iter().collect::<String>();


    for ch in s.chars() {
        print!("'{}', ", ch.to_lowercase());
    }
    println!();
    
    println!("ts now: {} -> {}", ts, to_base62(ts));
    println!("ts max: {} -> {}", tmax, to_base62(tmax));
}

