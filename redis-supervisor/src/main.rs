use anyhow::Result;
use log::{error, info};
use std::fs::File;
// use std::time::Duration;
use subprocess::{Exec, Redirection};

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn start_redis(port: u32) -> Result<()> {
    // read the redis.conf template

    info!("start instance on port: {}, ...", port);
    let filename = &format!("logs/out-{}.log", port);
    let fout = File::create(filename)?;

    let instance_folder = "instances";
    let redis_conf = format!("redis-{}.conf", port);

    let p = Exec::cmd("redis-server")
        .arg(redis_conf)
        .cwd(instance_folder)
        .stdout(Redirection::File(fout))
        .stderr(Redirection::Merge)
        .popen()?
        .wait()?;

    if p.success() {
        info!("exit success? {}", p.success(),);
    } else {
        error!("exit status failed, see log file: {}", filename);
    }

    Ok(())
}

fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("read the command line args...");
    // read config

    info!("read the supervisor config file...");
    // let config = std::fs::read_to_string()

    // determine if any other supervisors are running;
    // if so, determine the leader, else set leader to me

    // verify the supervisor directory structure is in place

    // verify any current instances that are up

    // start instances if necessary

    start_redis(2001)?;
    start_redis(2002)?;
    start_redis(2003)?;

    // begin supervisor loop with a ping to the database to ensure it stays alive and healthy

    Ok(())
}
