#!/usr/bin/env rust-script
// cargo-deps: async-io, async-process, futures, async-std
//
//! demonstrates how to run system commands concurrently and
//! wait for completion in a single join function. join, and
//! join3 .. join5 plus join_all from sutures::future.
//!
//! @see https://docs.rs/async-process/1.6.0/async_process/
//! @see https://docs.rs/futures/0.3.5/futures/future/struct.JoinAll.html

use async_std::task;
use async_process::{Command, Stdio};
use futures::{future, io::BufReader, prelude::*};
use std::time::{SystemTime, UNIX_EPOCH};

async fn find(path: &str) -> Vec<String> {
    let mut child = Command::new("find")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn().unwrap();

    // future::yield_now().await;

    let mut list = vec![];
    list.push(format!("{} started {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), path));
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    list.push(format!("{} done {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), path));

    while let Some(line) = lines.next().await {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();

        list.push(format!("{} {}", now, line.unwrap()));
    }

    list
}

fn main() {
    async_io::block_on(async {
        let j3 = task::spawn(find("/tmp"));
        let j1 = task::spawn(find("../scripts"));
        let j2 = task::spawn(find("../tm-cli/src"));

        let jlist = future::join3(j1, j2, j3);
        let (r1, r2, r3) = jlist.await;

        println!("{:?}", r1);
        println!("{:?}", r2);

        let mut full = r1;
        full.extend(r2);
        full.extend(r3);

        full.sort();

        for item in full {
            println!("{}", item);
        }
    });
}
