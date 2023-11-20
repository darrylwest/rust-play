#!/usr/bin/env rust-script

use std::{fs, io};

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let paths = if args.length

    let path = "./data";

    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    for file in &entries {
        println!("{:?} {} {}", file, file.is_file(), file.is_symlink());
    }

    

    Ok(())
}
