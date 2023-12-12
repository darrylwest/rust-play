#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::{
    io::Write,
    fs::File,
};

fn main() -> Result<()> {
    let filename = "/tmp/my-foo-file.txt";
    let mut buf = File::create(filename)?;

    buf.write_all(b"this is a string of bytes\nwith line-feeds\n\r")?;
    buf.write_all(b"this is the second line")?;

    println!("file written to {}", filename);

    Ok(())
}
