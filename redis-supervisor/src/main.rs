use anyhow::Result;
use log::{error, info};
use std::{fs::File, io::Read};
// use std::time::Duration;
use subprocess::{Exec, Redirection};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub base_port: u16,
    pub instance_count: u8,
    pub redis_template: String,
}

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
fn start_redis(port: u16) -> Result<()> {
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

fn read_config(filename: &str) -> Result<Config> {
    let mut file = File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let config: Config = toml::from_str(&text).unwrap();

    Ok(config)
}


fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("read the command line args...");
    // read config

    info!("read the supervisor config file...");

    let config_filename = "config/supervisor.toml";
    let config = read_config(config_filename)?;
    info!("parsed {} config, version: {} ", config_filename, config.version);

    // determine if any other supervisors are running;
    // if so, determine the leader, else set leader to me

    // verify the supervisor directory structure is in place

    // verify any current instances that are up

    // start instances if necessary

    for p in 1..=config.instance_count {
        // create and write out the redis config file; or pipe to process
        start_redis(config.base_port + (p as u16))?;
    }

    // begin supervisor loop with a ping to the database to ensure it stays alive and healthy

    Ok(())
}
