#!/usr/bin/env rust-script
// cargo-deps: libm

fn main() {
  let n: f64 = 10.0;

  println!("log({}) = {}", n, libm::log10(n));

  let n: f64 = 20.0;

  println!("log({}) = {}", n, libm::log10(n));
}
