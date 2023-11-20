#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs;

fn get_meta(src: &str) {
    let meta = fs::metadata(src);

    println!("{:#?}", meta);
}

fn main() -> Result<()> {
    get_meta("./file-exists.rs");
    get_meta("./data/user.json");
    get_meta("./data/my_user.json");
    // get_meta("./file-not-exists.rs");

    Ok(())
}
