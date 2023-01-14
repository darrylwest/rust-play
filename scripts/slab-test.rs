#!/usr/bin/env rust-script
//!
//! ```cargo
//! [dependencies]
//! slab = "0.4.7"
//! ```
//!
//! Pre-allocated storage for a uniform data type: provides pre-allocated storage for a 
//! single data type. If many values of a single type are being allocated, it can be more 
//! efficient to pre-allocate the necessary storage. Since the size of the type is uniform, 
//! memory fragmentation can be avoided. Storing, clearing, and lookup operations become 
//! very cheap.
//!
//! @see https://crates.io/crates/slab or https://docs.rs/slab/latest/slab/

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
