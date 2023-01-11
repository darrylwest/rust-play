#!/usr/bin/env rust-script
// cargo-deps: async-io, async-process, futures-lite

use async_process::{Command, Stdio};
use futures_lite::{future, io::BufReader, prelude::*};
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
        let fut2 = find("../tm-cli/src");

        let fut1 = find("../scripts");


        let results = future::zip(fut2, fut1).await;

        let mut full = results.0;
        full.extend(results.1);

        full.sort();

        for item in full {
            println!("{}", item);
        }
    });
}
