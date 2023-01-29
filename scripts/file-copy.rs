#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let src = "data/user.json";
    let dest = "/tmp/data.json";

    println!("copy {src} to {dest} ...");

    fs::copy(src, dest)?;


    Ok(())
}
