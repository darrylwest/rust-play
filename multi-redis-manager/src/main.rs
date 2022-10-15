use anyhow::Result;
use std::time::Duration;
use subprocess::{Exec, Redirection};
use std::fs::File;

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn run_redis(port: u32) -> Result<()> {
    println!("start instance on port: {}", port);
    let folder = "logs";
    let filename = format!("{}/out-{}.log", folder, port);
    let fout = File::create(filename)?;

    let mut p = Exec::cmd("redis-server")
        .arg("config/redis.conf")
        .cwd("instances")
        .stdout(Redirection::File(fout))
        .stderr(Redirection::Merge)
        .popen()?;

    if let Some(pid) = p.pid() {
        println!("process running, pid: {}", pid)
    }

    let mut count = 0;
    loop {
        if let Some(status) = p.wait_timeout(Duration::new(10, 0))? {
            println!("status: {:?}", status);
        }

        if let Some(pid) = p.pid() {
            println!("{}: process running, pid: {}", count, pid)
        } else {
            println!("process is dead...");
            break;
        }

        count += 1;
    }

    // make sure that you send a `save` command...

    p.kill()?;
    p.wait()?;

    println!("process killed");

    Ok(())
}

fn main() -> Result<()> {
    // read config
    // let config = std::fs::read_to_string()
    // start the instances with async (begin with simple threads)

    run_redis(2001)?;

    // add shutdown logic with messaging

    Ok(())
}
