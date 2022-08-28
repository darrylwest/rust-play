#!/usr/bin/env rust-script

use std::collections::HashMap;

fn main() {
  let mut letters = HashMap::new();

  let s = "this is a test magazine article with lots and lots and lots of messages";

  for ch in s.chars() {
    if ch != ' ' {
      letters.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
  }

  for (key, val) in letters.iter() {
    println!("{} count: {}", key, val);
  }

  let mut words = HashMap::new();
  for word in s.split_whitespace() {
      let count = words.entry(word).or_insert(0);
      *count += 1;
  }

  println!("Word count: {:?}", words);
}
