#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs;

fn read_string() -> Result<()> {
    let sbuf = fs::read_to_string("./file-reader.rs")?;

    println!("file as string {}", sbuf);

    Ok(())
}

fn read_u8() -> Result<()> {
    let vbuf = fs::read("./file-reader.rs")?;
    println!("file as vec {:?}", vbuf);

    Ok(())
}

fn main() -> Result<()> {
    read_string()?;
    read_u8()?;

    Ok(())
}

