#!/usr/bin/env rust-script
// cargo-deps: rand

#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddr {
  fn show(&self) {
    match self {
      IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
      IpAddr::V6(str) => println!("{}", str),
    }
  }
}

fn main() {
  let home = IpAddr::V4(127, 0, 0, 1);

  let loopback = IpAddr::V6(String::from("::1"));

  println!("{:?} {:?}", home, loopback);

  home.show();
  loopback.show();
}
