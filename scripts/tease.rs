#!/usr/bin/env rust-script

fn main() {
    let a = 2;
    let b = 3;
    let c = | x | b | a | x;

    println!("{}", c(1));
}
