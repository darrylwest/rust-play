#!/usr/bin/env rust-script

use std::collections::HashMap;

//
// tests for new, clone, insert, get, remove...
//
fn main() {
    let map: HashMap<i32, i32> = (0..1_000_000).map(|x| (x, x * 2)).collect();

    let keys = map.keys();

    println!("map size: {} keys", keys.count());
}
