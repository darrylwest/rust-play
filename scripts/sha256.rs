#!/usr/bin/env rust-script
// cargo-deps: sha256

use sha256::digest_file;
use std::path::Path;
use std::env;

fn main() {

    let count = env::args().len();
    if count == 1 {
        eprintln!("!Error: Use: sha256 file [file ...]");
        return;
    }

    for arg in env::args().skip(1) {
        let name = &arg;
        let filename = Path::new(name);
        let val = digest_file(filename).unwrap();

        if count == 2 {
            println!("{}", val);
        } else {
            println!("{} {}", val, name);
        }
    }
}
