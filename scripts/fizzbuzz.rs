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
        let mod5 = i % 5 == 0;
        let mod3 = i % 3 == 0;

        if mod3 && mod5 {
            results.push(String::from("FizzBuzz"));
        } else if mod3 {
            results.push(String::from("Fizz"));
        } else if mod5 {
            results.push(String::from("Buzz"));
        } else {
            results.push(i.to_string());
        }

        i = i + 1;
    }
    
    results
}
