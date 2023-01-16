use anyhow::Result;
use std::env;

/// a url safe base64 encode/decode with no padding
pub mod url_b64 {
    use anyhow::Result;
    use base64::{engine::general_purpose, Engine as _};

    pub fn encode(data: &[u8]) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }

    pub fn decode(data: &str) -> Result<Vec<u8>> {
        let v = general_purpose::URL_SAFE_NO_PAD.decode(data)?;

        Ok(v)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let data: &[u8] = if args.is_empty() {
        b"my data stream is every WHERE"
    } else {
        args[0].as_bytes()
    };
    let encoded = url_b64::encode(data);

    println!("{}", encoded);

    let decoded = url_b64::decode(&encoded)?;
    println!("{}", String::from_utf8(decoded)?);

    Ok(())
}
