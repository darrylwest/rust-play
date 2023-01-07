/// a collection of async file utilities
use anyhow::Result;
use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;
use log::info;

pub async fn read_file(path: String) -> Result<String> {
    let tid = task::current().id();
    // let tname = task::current().name();

    let mut file = File::open(&path).await?;
    let mut text = vec![];
    let mut buf = vec![0u8; 16 * 256];

    info!("[{}] File '{}' open and reading...", tid, path);

    loop {
        let n = file.read(&mut buf).await?;

        if n == 0 {
            info!("[{}] File read complete...", tid);
            return Ok(String::from_utf8(text)?);
        }

        text.write_all(&buf[..n]).await?;
    }
}
