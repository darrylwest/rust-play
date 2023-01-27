#!/usr/bin/env rust-script

use std::fs;

fn main() -> std::io::Result<()> {
    let sbuf = fs::read_to_string("./file-reader.rs")?;

    println!("file as string {}", sbuf);

    let vbuf = fs::read("./file-reader.rs")?;
    println!("file as vec {:?}", vbuf);

    Ok(())
}
