use anyhow::Result;

use base64::{alphabet, engine, Engine as _};

fn main() -> Result<()> {

    let alpha = "!_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let alphabet = alphabet::Alphabet::new(alpha).unwrap();

    println!("{:?}", alpha);

    // a very weird config that encodes with padding but requires no padding when decoding...?
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(false)
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

    // let text = "ABCDEFGHIJK abcdefghij 123 456 789";
    let text = "The quick brown fox runs and runs over the red fence and into a big wall.";
    let enc = crazy_engine.encode(text);

    println!("original: {}", text);
    println!("encoded : {}",  enc);
    
    let dec = crazy_engine.decode(enc).unwrap();
    let decs = String::from_utf8_lossy(&dec);
    
    println!("decoded : {}", decs);
    
    assert_eq!(text, decs);

    Ok(())
}
