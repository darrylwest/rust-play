
/// a collection of async file utilities
use anyhow::Result;
use log::info;
use async_std::fs::File;
use async_std::prelude::*;

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
