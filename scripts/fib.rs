#!/usr/bin/env rust-script

use std::io;

/*
fn fib_recurse(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recurse(n - 1) + fib_recurse(n - 2),
    }
}
*/


//
// fib defined as Fn = Fn-1 + Fn-2; sequence is 0, 1, 1, 2, 3, 5, 8, 13, 34, 55, 89, 144, ...
//
pub fn fib_standard(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;

            for _ in 1..n {
                let old = a;
                a = b;
                b += old;
            }

            b
        }
    }
}

fn main() {
    println!("Fibonacci generator");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "quit" {
            break;
        }

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib_standard(n));
    }
}

