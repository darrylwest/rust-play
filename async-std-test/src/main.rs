
// use std::env::args;
use anyhow::Result;
use log::info;
use std::time::Duration;
use async_std::{fs::File, prelude::*, task};

pub async fn read_file(path: String) -> Result<String> {
    let mut file = File::open(&path).await?;
    // let mut stdout = io::stdout();
    let mut text = vec![];
    let mut buf = vec![0u8; 16 * 256];

    info!("[t] File '{}' open and reading...", path);

    loop {
        let n = file.read(&mut buf).await?;

        if n == 0 {
            info!("[t] File read complete...");
            return Ok(String::from_utf8(text)?);
        }

        text.write_all(&buf[..n]).await?;
    }
}

fn main() -> Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let path = "Cargo.toml";
    let task = task::spawn(read_file(path.to_string()));

    task::block_on(task::sleep(Duration::from_millis(500)));

    info!("[m]reading file: {}", path);

    let r = task::block_on(task)?;

    info!("[m]file read complete, {} bytes.", r.len());
    // info!("[m]\n{}", r);

    Ok(())
}
