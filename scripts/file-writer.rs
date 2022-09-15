#!/usr/bin/env rust-script

use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let filename = "/tmp/my-foo-file.txt";
    let mut buf = File::create(filename)?;

    buf.write_all(b"this is a string of bytes\nwith line-feeds\n\r")?;

    println!("file written to {}", filename);

    Ok(())
}
