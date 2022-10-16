use anyhow::Result;
use log::{error, info};
use serde_derive::Deserialize;
use std::time::Duration;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};
use subprocess::{Exec, Redirection};
// use redis::Client;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub base_port: u16,
    pub instance_folder: String,
    pub instance_count: u8,
    pub redis_template: String,
    pub startup_delay_seconds: u64,
    pub ping_loop_interval_seconds: u64,
    pub ping_loop_limit: u16, // zero = infinate...
}

impl Config {
    pub fn auth(&self) -> String {
        // TODO figure out a better way to get this key; or create on the fly?...
        "65ba104a9e3eef7a655c9027fdf59e27".to_string()
    }
}

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

// create a struct for this
pub fn start_redis(port: u16) -> Result<()> {
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
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let config: Config = toml::from_str(&text).unwrap();

    Ok(config)
}

pub fn read_template(config: &Config, instance_no: u16) -> Result<String> {
    let file = File::open(&config.redis_template)?;
    let reader = BufReader::new(file);

    let mut text = String::new();
    let port = format!("{}", config.base_port + instance_no);
    let auth = config.auth();

    for (_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();

        if !line.starts_with('#') && !line.is_empty() {
            let line = line
                .replace("{{PORT}}", &port)
                .replace("{{PASSWORD}}", &auth);

            text.push_str(&line);
            text.push('\n');
        }
    }

    // will have to strip this out when template changes...
    assert!(text.len() > 1500);

    Ok(text)
}

fn write_redis_conf(text: &[u8], target: &str) -> Result<()> {
    info!("write config to {}", target);
    let mut file = File::create(target)?;
    file.write_all(text)?;

    Ok(())
}

fn main() -> Result<()> {
    log4rs::init_file("config/rolling.yaml", Default::default()).unwrap();

    info!("read the command line args...");
    // read config

    info!("read the supervisor config file...");

    let config_filename = "config/supervisor.toml";
    let config = read_config(config_filename)?;
    info!(
        "parsed {} config, version: {} ",
        config_filename, config.version
    );

    // determine if any other supervisors are running;
    // if so, determine the leader, else set leader to me

    // verify the supervisor directory structure is in place

    // verify any current instances that are up

    // start instances if necessary

    for n in 1..=config.instance_count {
        // create and write out the redis config file; or pipe to process
        let port = config.base_port + (n as u16);
        let text = read_template(&config, n as u16).unwrap();

        let redis_conf = format!("{}/redis-{}.conf", "instances", port);
        write_redis_conf(text.as_bytes(), &redis_conf)?;

        start_redis(port)?;
    }

    // delay a bit to give time for the instances to start...
    info!(
        "give the instances {} seconds to start up, then start the ping loop...",
        config.startup_delay_seconds
    );
    std::thread::sleep(Duration::from_secs(config.startup_delay_seconds));

    // begin supervisor loop with a ping to the database to ensure it stays alive and healthy
    let mut count = 0_u16;
    loop {
        std::thread::sleep(Duration::from_secs(config.ping_loop_interval_seconds));

        for n in 1..=config.instance_count {
            let port = config.base_port + (n as u16);
            info!("ping: 127.0.0.1:{}", port);

            let url = format!("redis://{}:{}@127.0.0.1:{}", "default", config.auth(), port);

            let client = redis::Client::open(url)?;
            let mut conn = client.get_connection()?;

            // test the connection
            let result: redis::RedisResult<()> = redis::cmd("PING").query(&mut conn);
            match result {
                Err(e) => error!("{}", e),
                _ => info!("ping ok"),
            }
        }

        if config.ping_loop_limit != 0 {
            count += 1;
            if count > config.ping_loop_limit {
                info!("breaking out of ping loop...");
                break;
            }
        }
    }

    std::thread::sleep(Duration::from_secs(1));
    info!("shut down the instances...");

    for n in 1..=config.instance_count {
        let port = config.base_port + (n as u16);
        info!("shutdown: 127.0.0.1:{}", port);

        let url = format!("redis://{}:{}@127.0.0.1:{}", "default", config.auth(), port);

        let client = redis::Client::open(url)?;
        let mut conn = client.get_connection()?;

        // ignore the error...
        let _: redis::RedisResult<()> = redis::cmd("SHUTDOWN").arg("SAVE").query(&mut conn);

        let result: redis::RedisResult<()> = redis::cmd("PING").query(&mut conn);
        match result {
            Err(e) => info!("ping after shutdown: {}", e),
            _ => error!("shutdown failed..."),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config_test() {
        let config: Config = read_config("config/supervisor.toml").unwrap();

        assert_eq!(config.base_port, 2000);
        assert_eq!(config.instance_folder, "instances");
        assert!(config.instance_count >= 3);
        assert_eq!(config.redis_template, "config/redis.conf.template");
    }

    #[test]
    fn read_template_test() {
        let config: Config = read_config("config/supervisor.toml").unwrap();
        // let port = 2001_u16;
        let text = read_template(&config, 1).unwrap();

        assert!(text.len() > 1500);
        // check for port number in file
        // println!("{}", text);
    }
}
