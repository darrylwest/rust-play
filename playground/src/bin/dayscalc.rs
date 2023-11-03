
use chrono::NaiveDate;

fn calculate_days(date1: &str, date2: &str) -> i64 {
    let a = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();
    let b = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();
    let duration = b.signed_duration_since(a);
    duration.num_days()
}

fn main() {
    let date1 = "1950-11-03";
    let date2 = "2023-11-03";

    println!("The number of days between {} and {} is {} days.", date1, date2, calculate_days(date1, date2));
}

