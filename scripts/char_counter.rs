#!/usr/bin/env rust-script

use std::collections::HashMap;

fn main() {
  let mut letters = HashMap::new();

  for ch in "this is a test magazine article with lots of messages".chars() {
    if ch != ' ' {
      letters.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
  }

  for (key, val) in letters.iter() {
    println!("{} count: {}", key, val);
  }
}
