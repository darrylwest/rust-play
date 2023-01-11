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


  println!("type of a: {} -> {}", a, type_of(a));
  println!("type of b: {} -> {}", b, type_of(b));
  println!("type of c: {} -> {}", c, type_of(c));
  println!("type of d: {} -> {}", d, type_of(d));
  println!("type of e: {} -> {}", e, type_of(e));
  println!("type of f: {} -> {}", f.clone(), type_of(f.clone()));
  println!("type of g: {} -> {}", g.clone(), type_of(g.clone()));
}
