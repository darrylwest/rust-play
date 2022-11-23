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

    let t1 = Instant::now();

    println!("t0: {:?}, t1: {:?}, t1 > t0? {}", t0, t1, t1 > t0);

}

fn show_weekday() {
    use chrono::prelude::*;
    // use chrono::Weekday;
    use chrono::naive::{NaiveDateTime, Days};

    let date = NaiveDate::from_ymd_opt(2022, 10, 20).unwrap();
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);

    // suitable for unix cron format of day of week (sunday = 0 or 7)
    println!("date: {:?}", dt);
    let dt = NaiveDateTime::parse_from_str("2022-11-20 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("date: {:?} {} {}", dt, dt.weekday(), dt.weekday().number_from_monday() % 7);

    for n in 1..7 {
        let dt = dt.checked_add_days(Days::new(n as u64)).unwrap();
        println!("date: {:?} {} {}", dt, dt.weekday(), dt.weekday().number_from_monday() % 7);
    }
}


fn main() {
    chrono_test();
    stdtime_test();
    show_weekday();

    println!("\nNano Second NOTE: this works in linux but osx only shows 000 for nanos.");
    println!("There is a fix in cpp if you want to ffi it in: ~/raincity/c-projects/cpp-utils/datetime\n");
}

