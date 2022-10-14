use anyhow::Result;
use subprocess::{Exec, Redirection};

fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn run_redis() -> Result<()> {
    let result = Exec::cmd("redis-server")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture();

    match result {
        Ok(data) => show_utf8(data.stdout),
        Err(e) => eprintln!("error: {}", e),
    }

    Ok(())
}

fn main() -> Result<()> {
    // read config
    // let config = std::fs::read_to_string()
    // start the instances with async (begin with simple threads)

    run_redis()?;

    // add shutdown logic with messaging

    Ok(())
}
