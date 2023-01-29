#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let src = "./file-exists.rs";
    let attr = fs::metadata(src)?;

    println!("file {} metadata: {:?}", src, attr);

    Ok(())
}
