#!/usr/bin/env rust-script
//! dpw@plaza.localdomain 2023-12-27 18:43:27
//!
//! ```cargo
//! [dependencies]
//! rand_chacha="0.3"
//! rand_core="0.7"
//! ```


use rand_chacha::rand_core::SeedableRng;
use rand_core::RngCore;
use rand_chacha::ChaChaRng;

fn main() {
    let mut gen = ChaChaRng::from_entropy();
    let res: Vec<u64> = (0..50).map(|_| gen.next_u64()).collect(); 
    
    for r in res {
        let s = format!("{:0x}", r);
        println!("{} {} {}", r, s, s.len());
    }
}

