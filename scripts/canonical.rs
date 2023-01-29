#!/usr/bin/env rust-script

#![allow(unused)]
use std::fs;

fn main() -> std::io::Result<()> {
    let path = fs::canonicalize("../scripts/../README.md")?;
    println!("{:?}", path);
    
    Ok(())
}
