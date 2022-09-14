#!/usr/bin/env rust-script
// cargo-deps: rand


fn factorial(n: u128) -> u128 {
    (1..n+1).product()
}

fn main() {
    println!("result: {}", factorial(20));
}
