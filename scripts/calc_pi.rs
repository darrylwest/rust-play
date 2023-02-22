#!/usr/bin/env rust-script
//! dpw@Darryls-iMac.localdomain 2023-02-22 19:07:18
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde = "1"
//! ```

use anyhow::Result;
// use std::env;

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().skip(1).collect();
    println!("pi: {}", std::f64::consts::PI);

    Ok(())
}

