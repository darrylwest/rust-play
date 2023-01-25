#!/usr/bin/env rust-script

fn main() {
    // the prices for apples, grapes and pears
    let prices = [3, 4, 2];

    // the qty sold by day
    let mon = [13, 8, 6];
    let tue = [9, 7, 4];
    let wed = [7, 4, 0];
    let thu = [15, 6, 3];
    let days = [mon, tue, wed, thu];

    // totals for the days
    let mut totals = [0, 0, 0, 0];

    for (d, day) in days.iter().enumerate() {
        for n in 0..prices.len() {
            totals[d] += prices[n] * day[n];
        }
    };

    let sum: i32 = totals.iter().sum();
    println!("{totals:?}, total: {sum}");
}

