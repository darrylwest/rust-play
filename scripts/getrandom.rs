#!/usr/bin/env rust-script
//! 
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! getrandom = "0.2.8"
//! ```
//! 

use anyhow::Result;

fn main() -> Result<()> {
    let mut buf = [0u8; 32];

    let _r = getrandom::getrandom(&mut buf).is_ok();

    println!("{:?}", buf);

    let mut val = String::new();
    buf.iter().for_each(|n| val.push_str(&format!("{:x}", n)));

    println!("{:?}", val);

    Ok(())
}
