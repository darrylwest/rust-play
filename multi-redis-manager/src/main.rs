use anyhow::Result;
use std::time::Duration;
use subprocess::{Exec, Redirection};

fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn run_redis() -> Result<()> {
    let mut p = Exec::cmd("redis-server")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .popen()?;

    if let Some(pid) = p.pid() {
        println!("process running, pid: {}", pid)
    }

    if let Some(status) = p.wait_timeout(Duration::new(5, 0))? {
        println!("process completed, status: {:?}", status);
    } else {
        println!("still running, pid: {:?}", p.pid());

        p.kill()?;
        p.wait()?;

        println!("process killed");
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
