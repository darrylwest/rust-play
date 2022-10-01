#!/usr/bin/env rust-script
// cargo-deps: chrono


fn chrono_test() {
    println!("chrono time tests:");

    use chrono::prelude::*;
    let utc: DateTime<Utc> = Utc::now();

    println!(
        "micros: {} nanos: {} {:x} {}",
        utc.timestamp_micros(),
        utc.timestamp_nanos(),
        utc.timestamp_micros(),
        utc.to_rfc3339()
    );

    let key = format!("{:X}", Utc::now().timestamp_micros());

    println!("key: {} length: {}", key, key.len());
}

fn stdtime_test() {
    println!("std::time tests:");

    use std::time::{SystemTime, Instant, UNIX_EPOCH};
    
    let t0 = Instant::now();
    let now = SystemTime::now();
    println!("t0: {:?} sys: {:?}\n", t0, now);
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            println!("{:?} duration", n);
            println!("{} nano seconds ago!", n.as_nanos());
            println!("{} micro seconds ago!", n.as_micros());
            println!("{} milli seconds ago!", n.as_millis());
            println!("{} seconds ago!", n.as_secs());
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }


}


fn main() {
    chrono_test();
    stdtime_test();

    println!("\nNano Second NOTE: this works in linux but osx only shows 000 for nanos.");
    println!("There is a fix in cpp if you want to ffi it in: ~/raincity/c-projects/cpp-utils/datetime\n");
}

