#!/usr/bin/env rust-script
//! dpw@Darryls-iMac.localdomain 2023-02-04 18:48:11
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! serde = "1"
//! ```

use anyhow::Result;
// use std::env;

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().skip(1).collect();
    // println!("args: {:?}", args);

    let info = "id=32 addr=127.0.0.1:56858 laddr=127.0.0.1:6450 fd=9 name= age=0 idle=1344 flags=N db=1 sub=0 psub=0 ssub=0 multi=-1 qbuf=26 qbuf-free=16864 argv-mem=10 multi-mem=0 rbs=16384 rbp=16384 obl=0 oll=0 omem=0 tot-mem=34042 events=r cmd=client|info user=default redir=-1 resp=2";

    println!("{}", info);
    let terms: Vec<&str> = info.split(' ').collect();
    println!("{:?}", terms);

    let idv: Vec<&str> = terms.clone().into_iter().filter(|&x| x.starts_with("id=")).collect();
    let id = idv[0].replace("id=", "").parse::<u16>().unwrap();
    
    let dbv: Vec<&str> = terms.clone().into_iter().filter(|&x| x.starts_with("db=")).collect();
    let db = dbv[0].replace("db=", "").parse::<u16>().unwrap();
    
    let dbv: Vec<&str> = terms.clone().into_iter().filter(|&x| x.starts_with("idle=")).collect();
    let idle = dbv[0].replace("idle=", "").parse::<u64>().unwrap();
    
    println!("id={}, db={}, idle={}", id, db, idle);
    
    assert_eq!(id, 32);
    assert_eq!(db, 1);


    Ok(())
}

