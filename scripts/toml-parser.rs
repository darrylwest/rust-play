#!/usr/bin/env rust-script
//! Use serde json to parse an un-typed object and display the contents
//! 
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! toml = "0.5"
//! 

use toml::value::Value;
// use toml::de::from_str;
// use serde_derive::Deserialize; // this is if you have a struct to deserialize into
use anyhow::Result;
use std::{
    fs,
    env,
    process,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Error! Use {} file [file, file, ...]", "toml-parser");
        process::exit(0x01);
    }

    for filename in args.iter() {
        let text = fs::read_to_string(filename)?;
        let value: Value = toml::from_str(&text)?;

        if args.len() > 1 {
            println!("Parsed: {}", filename);
        }

        println!("{}", &value);
    }

    Ok(())
}

