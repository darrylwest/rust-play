#!/usr/bin/env rust-script

use std::env;

// preferred method without clap
fn main() {
    let args: Vec<String>  = env::args().skip(1).collect();
    // println!("{:?}", args);

    for arg in args {
        println!("{}", arg);
    }
}

