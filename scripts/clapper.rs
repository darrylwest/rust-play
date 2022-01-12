#!/usr/bin/env rust-script
//!
//!
//! ```cargo
//! [dependencies]
//! clap = {version = "2.34", features = ["yaml"]}
//! ```

use clap::{Arg, App}; // , SubCommand};

fn main() {
    let matches = App::new("My Test Clap App")
        .version("1.3.4")
        .author("dpw")
        .about("this is a test application...")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to work on")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Sets the verbosity level"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("config file -> {}", config);

    println!("input file -> {}", matches.value_of("INPUT").unwrap());

    match matches.occurrences_of("v") {
        0 => println!("quiet"),
        1 => println!("verbose 1"),
        2 => println!("verbose 2"),
        3 | _ => println!("loud and crazy"),
    }
}

