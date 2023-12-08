use chrono::{DateTime, Utc};
use std::env;

fn unix2date(unix_timestamp: u64) -> String {
    let dt = DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(unix_timestamp as i64, 0).unwrap(),
        Utc,
    );
    dt.to_rfc3339()
}

fn parse_arg_to_u64(arg: &str) -> Result<u64, std::num::ParseIntError> {
    arg.parse::<u64>()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a unix timestamp in seconds.");
        return;
    }


    let arg = &args[1];
    let ts = match parse_arg_to_u64(arg) {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let ss = unix2date(ts);
    println!("{}", ss);
}
