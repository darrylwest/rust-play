#!/usr/bin/env rust-script

fn main() {
    let results = fizz_buzz(20);

    for str in results {
        println!("{}", str);
    }
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut results: Vec<String> = Vec::with_capacity(n as usize);
    let mut i: i32 = 1;
    
    while i <= n {
        if i % 3 == 0 && i % 5 == 0 {
            results.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            results.push("Fizz".to_string());
        } else if i % 5 == 0 {
            results.push("Buzz".to_string());
        } else {
            results.push(i.to_string());
        }

        i = i + 1;
    }
    
    results
}
