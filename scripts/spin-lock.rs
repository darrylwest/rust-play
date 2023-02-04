#!/usr/bin/env rust-script
//! dpw@Darryls-iMac.localdomain 2023-02-04 22:09:14
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde = "1"
//! ```

use anyhow::Result;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args: {:?}", args);

    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);

    let thread = thread::spawn(move || {
        println!("zero the clone");
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    let mut n = 0;
    while spinlock.load(Ordering::SeqCst) != 0 {
        n += 1;
        hint::spin_loop();
    }

    println!("spin loop count: {}", n);

    if let Err(panic) = thread.join() {
        println!("thread error: {panic:?}");
    }

    Ok(())
}

