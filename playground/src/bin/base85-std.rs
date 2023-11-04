use anyhow::Result;
use std::env;
use base85;

fn main() -> Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    let data: &[u8] = if args.is_empty() {
        b"my data stream is every WHERE"
    } else {
        args[0].as_bytes()
    };

    println!("input  : {:?}", String::from_utf8_lossy(data));
    let encoded = base85::encode(data);
    println!("encoded: {:?}", encoded);
    let decoded: Vec<u8> = base85::decode(&encoded)?;
    println!("decoded: {:?}", String::from_utf8_lossy(&decoded));

    Ok(())
}