
// use std::env::args;

use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;

pub async fn read_file(path: String) -> io::Result<String> {
    let mut file = File::open(&path).await?;
    // let mut stdout = io::stdout();
    let mut text = vec![];
    let mut buf = vec![0u8; 16 * 1024];

    loop {
        let n = file.read(&mut buf).await?;

        if n == 0 {
            return Ok(String::from_utf8(text).unwrap());
        }

        text.write_all(&buf[..n]).await?;
    }
}

fn main() -> io::Result<()> {
    let path = "Cargo.toml".to_string();

    println!("read file: {}", path);

    let r = task::block_on(read_file(path.to_string()))?;

    println!("{}", r);

    Ok(())
}
