#!/usr/bin/env rust-script
//! Use serde json to parse an un-typed object and display the contents
//! 
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde_json = "1"
//! 

use serde_json::Value;
use anyhow::Result;
use std::{
    fs,
    env,
    process,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Error! Use {} file [file, file, ...]", "serde-json-parser");
        process::exit(0x01);
    }

    for filename in args.iter() {
        let text = fs::read_to_string(filename)?;
        let value: Value = serde_json::from_str(&text)?;

        if args.len() > 1 {
            println!("Parsed: {}", filename);
        }

        println!("{}", &value);
    }

    Ok(())
}

