#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let src = Path::new("data/user.json");

    let dest = Path::join(Path::new("backup/your"), &src.file_name().unwrap());
    let parent = dest.parent().unwrap();

    if !parent.exists() {
        println!("mkdir {:?}", parent);
        if fs::create_dir_all(parent).is_err() {
            return Err(anyhow!("could not create parent folder: {}", parent.display()));
        }
    }

    println!("copy {} to {} ...", src.display(), dest.display());

    fs::copy(src, dest)?;


    Ok(())
}
