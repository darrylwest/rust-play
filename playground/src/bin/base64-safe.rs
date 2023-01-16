

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};


fn main() -> Result<()> {
    let data = b"my data stream is every WHERE";
    let encoded= general_purpose::URL_SAFE_NO_PAD.encode(data);

    println!("{}", encoded);

    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
    println!("{}", String::from_utf8(decoded)?);


    Ok(())
}