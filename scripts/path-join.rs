#!/usr/bin/env rust-script

use std::path::PathBuf;

fn main() {
    let mut pbuf: PathBuf = std::env::current_dir().unwrap();
    pbuf.push("foo");
    pbuf.push("bar/baz");

    println!("path: {}", pbuf.to_str().unwrap());

    // or
    let pbuf: PathBuf = [ "foo", "bar", "baz" ].iter().collect();
    println!("path: {}", pbuf.to_str().unwrap());

    for it in pbuf.iter() {
        println!("{:?}", it);
    }
}
