#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let src = Path::new("/root/bad-file.txt");
    println!("file {:?} exists? {}", src, src.exists());

    let src = Path::new("./file-exists.rs");
    println!("file {:?} exists? {}", src, src.exists());

    Ok(())
}
