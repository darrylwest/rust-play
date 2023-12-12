#!/usr/bin/env rust-script
// cargo-deps: anyhow

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut list: Vec<String> = Vec::new();

    for line in reader.lines() {
        list.push(line?);
    }

    Ok(list)
}

fn main() -> Result<()> {

    let lines = read_lines("./data/users.kv")?;
    println!("line count {}", lines.len());
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

