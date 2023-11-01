#!/usr/bin/env rust-script
// cargo-deps: base64

use base64::{Engine as _, engine::general_purpose};

fn main() {

    let orig = "this is a test string";
    println!("Encode this '{}' without padding then for url safe.", orig);

    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(orig);
    
    println!("enc: {}", encoded);
    
    let dec = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
    let decs = String::from_utf8_lossy(&dec);
    
    println!("dec: {}", decs);
    
    // or, URL-safe
    let enc_url = general_purpose::URL_SAFE_NO_PAD.encode(orig);
    println!("url: {}", enc_url);

    let dec_url = general_purpose::URL_SAFE_NO_PAD.decode(enc_url).unwrap();
    let decs = String::from_utf8_lossy(&dec_url);
    
    println!("dec: {}", decs);
}
