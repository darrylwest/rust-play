#!/usr/bin/env rust-script
// cargo-deps: rand

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

fn main() {
  let a = 23;
  let b = 0.4;
  let c = 44i64;
  let d = 55.0f32;
  let e = "this is a str";
  let f = String::from("this is a string");
  let g: Box<&str> = Box::new("this is a box test");


  println!("type of a: {} -> {}", type_of(a), a);
  println!("type of b: {} -> {}", type_of(b), b);
  println!("type of c: {} -> {}", type_of(c), c);
  println!("type of d: {} -> {}", type_of(d), d);
  println!("type of e: {} -> {}", type_of(e), e);
  println!("type of f: {} -> {}", type_of(f.clone()), f.clone());
  println!("type of g: {} -> {}", type_of(g.clone()), g.clone());
}
