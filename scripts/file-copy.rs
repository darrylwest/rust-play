#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let src = Path::new("data/user.json");

    let dest = Path::join(Path::new("backup/your"), &src.file_name().unwrap());
    let parent = dest.parent().unwrap();

    if !parent.exists() {
        let resp = fs::create_dir_all(parent);
        println!("mkdir resp: {:?}", resp);
    }

    println!("copy {} to {} ...", src.display(), dest.display());

    fs::copy(src, dest)?;


    Ok(())
}
