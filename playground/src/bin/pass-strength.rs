//! calc the password strength
//! 

use anyhow::Result;
use zxcvbn::zxcvbn;
use std::env;
use log::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Error: Use pass-strength password [password, ...]");
        std::process::exit(1);
    }

    info!("{:?}", args);

    for pw in args.iter() {
        let e = zxcvbn(pw, &[])?;

        println!("score:     {} / 4", e.score());
        println!("guesses:   {}", e.guesses());
        println!("calc time: {:?}", e.calculation_time());
    }

    Ok(())
}