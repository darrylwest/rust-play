//! calc all the factors for a given number.  return the table of valid factors
use anyhow::Result;
use std::env;

#[derive(Default, Debug, Clone)]
pub struct Factor {
    pub a: i32,
    pub b: i32,
}

fn show_help() {
    let s = r#"
Use: factors val

Where val is the positive integer to be factored.  Shows all combinations.

Version: 1.1
    "#;

    println!("{s}");
}

/// build a Factor table with every combination of factors between 
pub fn factor(num: i32) -> Vec<Factor> {
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

        x += 1;
    }

    list
}

/// display the factors row by row
fn show_factors(num: i32, list: Vec<Factor>) {
    let prime = list.len() == 1;

    let width = match num {
        0..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        _ => 5
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

    if args.len() < 1 {
        show_help();
        return Ok(());
    }

    match args[0].parse::<i32>() {
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
}