/// time-ops covers most of the generation and conversion of unix time stamps to rfc3339 formats
/// should port this to a library and eventually a crate
/// datetime-rs

use chrono::{NaiveDate, Utc, TimeZone, NaiveDateTime};

pub struct TimeOps {
    pub start_time: u64,
}

impl TimeOps {

    /// return the utc unix timestamp
    pub fn timestamp() -> u64 {
        Utc::now().timestamp() as u64
    }

    /// returns the current utc time in millis
    pub fn now_millis() -> u64 {
        Utc::now().timestamp_millis() as u64
    }

    /// returns the current utc time in micros
    pub fn now_micros() -> u64 {
        Utc::now().timestamp_micros() as u64
    }

    /// returns the current utc time in nanos
    pub fn now_nanos() -> u64 {
        Utc::now().timestamp_nanos_opt().unwrap() as u64
    }

    /// return the rfc3339 formated string from the timestamp
    pub fn rfc3339_from_timestamp(ts: u64) -> String {
        Utc.timestamp_opt(ts as i64, 0).unwrap().to_rfc3339()
    }

    /// return the current naive date
    pub fn today() -> NaiveDate {
        Utc::now().date_naive()
    }

    /// return the current utc date time
    pub fn utc_now() -> NaiveDateTime {
        Utc::now().naive_utc()
    }
}

fn main() {
    let ts = TimeOps::timestamp();
    println!("Unix UTC timestamp: {}", ts);
    println!("millis: {}", TimeOps::now_millis());
    println!("micros: {}", TimeOps::now_micros());
    println!("nanos : {}", TimeOps::now_nanos());
    println!("rfc3339 from timestamp: {}", TimeOps::rfc3339_from_timestamp(ts));
    println!("utc today: {}", TimeOps::today());
    println!("utc now  : {}", TimeOps::utc_now());
}

