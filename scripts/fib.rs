#!/usr/bin/env rust-script

use std::collections::HashMap;
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

fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let v = match n {
        0 => 0,
        1 => 1,
        _ => fib_memo(n - 2, memo) + fib_memo(n - 1, memo),
    };

    memo.insert(n, v);
    v
}

//
// fib defined as Fn = Fn-1 + Fn-2; sequence is 0, 1, 1, 2, 3, 5, 8, 13, 34, 55, 89, 144, ...
//
fn fib_standard(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => a,
        1 => b,
        _ => {
            for _ in 1..n {
                let old = a;
                a = b;
                b += old;
            }

            b
        }
    }
}

// let v = fib_iter().take(20).collect::<Vec<_>>();
//
fn fib_iter() -> impl Iterator<Item=u64> {
    let mut state = (0, 1);

    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn main() {
    println!("Fibonacci generator(s)");
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

        println!("std : {}", fib_standard(n));

        let mut memo = HashMap::new();
        println!("memo: {}", fib_memo(n, &mut memo));

        let v = fib_iter().take(n as usize).collect::<Vec<_>>();
        println!("iter: {:?}", v);
    }
}
