#!/usr/bin/env rust-script
// cargo-deps: chrono

use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();

    println!("{} {:x} {}", utc.timestamp(), utc.timestamp(), utc.to_rfc3339());
}
