#!/usr/bin/env rust-script

use std::collections::HashMap;

//
// tests for new, clone, insert, get, remove...
//
fn main() {
    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 2)).collect();
    let cloned = map.clone();

    let mut keys = map.keys().copied().collect::<Vec<_>>();
    keys.sort();
    for k in keys {
        print!("{} -> {}", k, map.get(&k).unwrap());
        println!(", clone: {} -> {}", k, cloned.get(&k).unwrap());
    }

    println!(
        "Map length: {}, capacity: {}, cloned: {}, {}",
        map.len(),
        map.capacity(),
        cloned.len(),
        cloned.capacity(),
    );

    println!("Remove: {}", map.remove(&3).unwrap());
    println!("New length: {}, capacity: {}", map.len(), map.capacity());

    map.shrink_to_fit();
    println!("Shrink length: {}, capacity: {}", map.len(), map.capacity());
}
