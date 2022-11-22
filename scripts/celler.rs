#!/usr/bin/env rust-script

use std::collections::HashMap;
use std::cell::Cell;

//
// tests for new, clone, insert, get, remove...
//
fn main() {

    let map = create_map();

    let cell = map.get(&100u8).unwrap();

    println!("{:?} {:?}", &map, &cell);

    cell.set(20);

    println!("{:?} {:?}", &map, &cell);

    println!("isize max: {}", isize::MAX);
    println!("usize max: {}", usize::MAX);
    println!("u64   max: {}", isize::MAX);
    println!("u128  max: {}", u128::MAX);
}

fn create_map() -> HashMap<u8, Cell<u8>> {
    let mut mmap: HashMap<u8, Cell<u8>> = HashMap::new();
    let cell = Cell::new(100);
    mmap.insert(cell.get(), cell.clone());

    mmap.clone()
}
