//! calc all the factors for a given number.  return the table of valid factors
use anyhow::Result;
use std::env;

#[derive(Default, Debug, Clone)]
pub struct Factor {
    pub a: i64,
    pub b: i64,
}

fn show_help() {
    let s = r#"
Use: factors val

Where val is the positive integer to be factored.  Shows all combinations.

Version: 1.2
    "#;

    println!("{s}");
}

/// build a Factor table with every combination of factors between 
pub fn factor(num: i64) -> Vec<Factor> {
    let mut list = vec![];

    let mut x = 1;
    while x < num {
        if num % x == 0 {
            let val = Factor{
                a: x,
                b: num / x,
            };

            // bail out if we have already shown this
            if val.a > val.b {
                break;
            }

            list.push(val);
        }

        x += 1i64;
    }

    list
}

/// display the factors row by row
fn show_factors(num: i64, list: Vec<Factor>) {
    let prime = list.len() == 1;

    let width = match num {
        0..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        100000..=999999 => 6,
        _ => 7
    };

    println!("Factors of {}", num);

    for f in list.into_iter() {
        println!("{:width$} * {:width$}", f.a, f.b);
    }

    if prime {
        println!("{} is a prime number.", num);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        show_help();
        return Ok(());
    }

    match args[0].parse::<i64>() {
        Ok(num) => {
            let list = factor(num);
            show_factors(num, list);
        },
        _ => show_help(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let list = factor(120);

        println!("{:?}", list);
    }

    #[test]
    fn factor_big() {
        let list = [ 485031120072_i64 ];

        for n in list {
            let list = factor(n);

            println!("{:?}", list);
        }
    }
}