#!/usr/bin/env rust-script

use std::collections::HashMap;

fn main() {
  let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x*2)).collect();

  let mut keys = map.keys().copied().collect::<Vec<_>>();
  keys.sort();
  for k in keys {
    println!("{} -> {}", k, map.get(&k).unwrap());
  }

  println!("Map length: {}, capacity: {}", map.len(), map.capacity());

  println!("Remove: {}", map.remove(&3).unwrap());
  println!("New length: {}, capacity: {}", map.len(), map.capacity());

  map.shrink_to_fit();
  println!("Shrink length: {}, capacity: {}", map.len(), map.capacity());
}

