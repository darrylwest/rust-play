#!/usr/bin/env rust-script
// cargo-deps: fastrand

fn main() {
    let n = fastrand::u64(..1_000_000);
    println!("{:06}", n);
}
