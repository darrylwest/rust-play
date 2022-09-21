#!/usr/bin/env rust-script
// cargo-deps: chrono

use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();

    println!(
        "{} {:x} {}",
        utc.timestamp_micros(),
        utc.timestamp_micros(),
        utc.to_rfc3339()
    );

    let key = format!("{:X}", Utc::now().timestamp_micros());

    println!("key: {} length: {}", key, key.len());
}
