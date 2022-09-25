#!/usr/bin/env rust-script

#![allow(unused)]

use std::io::Write;
use std::fs::File;

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0f64;
    for i in 0..N {
        sum += a[i] * b[i];
    }

    sum
}

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

    let n = dot_product([0.2, 1.3, 0.3], [3.3, 2.0, 4.0]);
    println!("n {}", n);
}
