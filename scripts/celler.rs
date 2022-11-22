#!/usr/bin/env rust-script

use std::collections::HashMap;
use std::cell::Cell;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

//
// tests for new, clone, insert, get, remove...
//
fn main() {
    let (tx, rx) = mpsc::channel();

    // wont compile
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        let map = create_map();
        let cell = map.get(&100u8).unwrap();

        println!("in thread: {:?} {:?}", &map, &cell);
        tx.send(map.clone()).unwrap();

        println!("in thread: {:?}", &cell);
        cell.set(20);
        println!("in thread: {:?} {:?}", &map, &cell);

        tx.send(map.clone()).unwrap();
    });


    let mp = rx.recv().unwrap();
    println!("main: {:?}", mp);

    let mp = rx.recv().unwrap();
    println!("main: {:?}", mp);
}

fn create_map() -> HashMap<u8, Cell<u8>> {
    let mut mmap: HashMap<u8, Cell<u8>> = HashMap::new();
    let cell = Cell::new(100);
    mmap.insert(cell.get(), cell.clone());

    mmap.clone()
}

pub fn type_sizes() {
    println!("isize max: {}", isize::MAX);
    println!("usize max: {}", usize::MAX);
    println!("u64   max: {}", isize::MAX);
    println!("u128  max: {}", u128::MAX);
}

