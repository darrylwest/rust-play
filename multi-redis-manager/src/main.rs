use anyhow::Result;
use std::fs::File;
// use std::time::Duration; // used for timeouts...
use subprocess::{Exec, Redirection};

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn start_redis(port: u32) -> Result<()> {
    println!("start instance on port: {}", port);
    let filename = format!("logs/out-{}.log", port);
    let fout = File::create(filename)?;

    let instance_folder = "instances";
    let redis_conf = format!("redis-{}.conf", port);

    let p = Exec::cmd("redis-server")
        .arg(redis_conf)
        .cwd(instance_folder)
        .stdout(Redirection::File(fout))
        .stderr(Redirection::Merge)
        .popen()?;

    if let Some(pid) = p.pid() {
        println!("process running, pid: {}", pid)
    }

    Ok(())
}

fn main() -> Result<()> {
    // read config
    // let config = std::fs::read_to_string()
    // start the instances with async (begin with simple threads)

    start_redis(2001)?;

    // add shutdown logic with messaging

    // TODO loop with a ping to the database to ensure it stays alive and healthy

    Ok(())
}
