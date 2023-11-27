//! parse the cli for numbers and check if for prime.

use anyhow::Result;
use std::env;

fn show_help() {
    let s = r#"
USE: isprime n [n, ...]

Example: isprime 19501103 22
19501103 is prime: true
22 is prime: false

gcd Version: 1.3, rcs/dpw
    "#;

    println!("{s}");
}

// sieve of eratosthenes
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return Ok(());
    }
    
    for arg in args.iter() {
        let n = arg.parse::<u64>().expect("this should parse to u64");
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

    #[test]
    fn big_prime() {
        let list = [ 485031120071u64, 398964368629 ];
        for n in list {
            assert_eq!(is_prime(n), true);
        }
    }
}