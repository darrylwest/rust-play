#!/usr/bin/env rust-script

#![allow(unused)]

use std::io::Write;
use std::fs::File;

pub fn say_hi<W: Write>(out: &mut W) -> std::io::Result<()>  {
    let _ = out.write(b"hello with more text\n");
    out.flush()
}

pub fn write_bytes(out: &mut dyn Write) -> std::io::Result<()> {
    let _ = out.write(b"hello with more text from the byte writer\n");
    out.flush()
}

fn show(buf: Vec<u8>) {
    buf.into_iter().map(|x| x as char).for_each(|ch| print!("{}", ch));
}

fn test_trait_object() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;

    let _ = writer.write_all(b"this is a block of text\n");
    show(buf);
}

fn main() {
    test_trait_object();

    let mut buf: Vec<u8> = vec![];
    write_bytes(&mut buf);
    show(buf);

    let mut fout = File::create("/tmp/hello.txt").unwrap();
    write_bytes(&mut fout);
}
