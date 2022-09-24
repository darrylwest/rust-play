#!/usr/bin/env rust-script

use std::io::Write;
// use std::fs::File;

// fn say_hello<W: Write() -> std::io::Result<()> 

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;

    writer.write_all(b"this is a block of text\n");

    buf.into_iter().map(|x| x as char).for_each(|ch| print!("{}", ch));


}
