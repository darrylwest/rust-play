
// use std::env::args;
use anyhow::Result;
use async_std::{fs::File, prelude::*, task};

pub async fn read_file(path: String) -> Result<String> {
    let mut file = File::open(&path).await?;
    // let mut stdout = io::stdout();
    let mut text = vec![];
    let mut buf = vec![0u8; 16 * 256];

    loop {
        let n = file.read(&mut buf).await?;

        if n == 0 {
            return Ok(String::from_utf8(text)?);
        }

        text.write_all(&buf[..n]).await?;
    }
}

fn main() -> Result<()> {
    let path = "Cargo.toml";
    let task = task::spawn(read_file(path.to_string()));
    println!("reading file: {}", path);

    let r = task::block_on(task)?;

    println!("file read complete, {} bytes.", r.len());
    println!("{}", r);

    Ok(())
}
