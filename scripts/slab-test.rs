#!/usr/bin/env rust-script
//!
//! ```cargo
//! [dependencies]
//! slab = "0.4.7"
//! ```

use slab::Slab;
// use slab::IntoIter;

fn main() {
    let mut slab = Slab::new();

    let a = slab.insert("howdy");

    println!("key: {:?} value: {}", a, slab[a]);

    let entry = slab.vacant_entry();
    let key = entry.key();

    println!("entry: {:?}, key: {}", entry, key);

    let b = slab.insert("doody");
    println!("key: {:?} value: {}", b, slab[b]);

    println!("slab: {:?}", &slab);

}
