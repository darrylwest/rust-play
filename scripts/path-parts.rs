#!/usr/bin/env rust-script

use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let path = Path::new("home/queue/email.txt");

    let parent = path.parent().unwrap().to_str().unwrap();
    let filename = path.file_name().and_then(OsStr::to_str).unwrap();
    let extension = path.extension().and_then(OsStr::to_str).unwrap();

    println!("Path  : {}", path.display());
    println!("Parent: {}", parent);
    println!("Name  : {}", filename);
    println!("Ext   : {}", extension);
}

