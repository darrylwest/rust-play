//! parse the cli for numbers and check if for prime.

use anyhow::Result;
use std::env;

fn show_help() {
    let s = r#"
USE: isprime n [n, ...]

Example: isprime 19501103 22
19501103 is prime: true
22 is prime: false

gcd Version: 1.0.1, rcs/dpw
    "#;

    println!("{s}");
}


fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        _ => !(2..n).any(|d| n % d == 0),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return Ok(());
    }
    
    for arg in args.iter() {
        let n = u64::from_str_radix(arg, 10)?;
        println!("{} is prime: {}", n, is_prime(n));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_list() {
        let list = [2, 3, 5, 7, 11, 13, 17, 19501103];
        for n in list {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn not_prime_list() {
        let list = [2, 3, 5, 7, 11, 13, 17];
        for n in list {
            let nn = n * 3;
            assert_eq!(is_prime(nn), false);
        }

    }
}