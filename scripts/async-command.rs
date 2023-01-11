#!/usr/bin/env rust-script
// cargo-deps: async-process, futures-lite

use async_process::{Command, Stdio};
use futures_lite::{future, io::BufReader, prelude::*};

fn main() {
    println!("hi");

    future::block_on(async {
        let mut child = Command::new("find")
            .arg(".")
            .stdout(Stdio::piped())
            .spawn().unwrap();

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }
    });
}
