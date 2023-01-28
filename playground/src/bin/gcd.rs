//! calc the greatest common divisor between two numbers, a, b
//!
use anyhow::Result;
use num::integer::Integer;
use std::env;

fn show_help() {
    let s = r#"
USE: gcd a b

Example: gcd 12 8
gcd 12, 8 = 4, prod = 96, lcm = 24

gcd Version: 1.0.1, rcs/dpw
    "#;

    println!("{s}");
}

fn calc(a: i128, b: i128) -> String {
    let c = a.gcd(&b);

    let prod = a * b;
    let lcm = prod / c;

    format!("gcd {a}, {b} = {c}; prod = {prod}; lcm = {lcm}")
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        show_help();
        return Ok(());
    }

    let a = args[0].parse::<i128>()?;
    let b = args[1].parse::<i128>()?;

    let r = calc(a, b);

    println!("{r}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_12_8() {
        let s = calc(12, 8);
        assert_eq!(s, "gcd 12, 8 = 4; prod = 96; lcm = 24");
    }

    #[test]
    fn calc_15_40() {
        let s = calc(15, 40);
        assert_eq!(s, "gcd 15, 40 = 5; prod = 600; lcm = 120");
    }
}
