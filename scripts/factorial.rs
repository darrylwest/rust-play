#!/usr/bin/env rust-script
// cargo-deps: rand

fn main() {
    let mut result: f64 = 1.0;

    for n in (2..21).rev() {
        result = result * (n as f64);
        println!("{}:{}", n, result);
    }

    println!("result: {}", result);
}
