#!/usr/bin/env rust-script
//! dpw@Darryls-iMac.localdomain 2023-02-04 21:55:15
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde = "1"
//! ```

use anyhow::Result;
use std::env;
use std::sync::atomic::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args: {:?}", args);

    let a = AtomicUsize::new(5);
    println!("{a:?}");

    a.store(100, Ordering::SeqCst);
    println!("{a:?}");

    a.fetch_add(1_usize, Ordering::SeqCst);
    println!("{a:?}");

    assert_eq!(a.load(Ordering::SeqCst), 101);

    a.fetch_add(1_usize, Ordering::SeqCst);
    println!("{a:?}");

    Ok(())
}

