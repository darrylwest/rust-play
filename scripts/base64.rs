#!/usr/bin/env rust-script
// cargo-deps: base64

use base64::{Engine as _, engine::general_purpose};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let text = match args.len() {
        0 => "this is a test string",
        _ => &args[0],
    };

    println!("\nEncode this '{}' without padding then for url safe.\n", text);

    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(text);
    
    println!("enc: {}", encoded);
    
    let dec = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
    let decs = String::from_utf8_lossy(&dec);
    
    println!("dec: {}", decs);
    
    // or, URL-safe
    let enc_url = general_purpose::URL_SAFE_NO_PAD.encode(text);
    println!("url: {}", enc_url);

    let dec_url = general_purpose::URL_SAFE_NO_PAD.decode(enc_url).unwrap();
    let decs = String::from_utf8_lossy(&dec_url);
    
    println!("dec: {}", decs);
}
