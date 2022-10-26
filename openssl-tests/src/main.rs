// use std::env;
// use num_bigint::BigUint;
// use rand::prelude::*;
use openssl::symm::{encrypt, decrypt, Cipher};
use std::iter::repeat_with;

fn gen_key() -> Vec<u8> {
    let rng = fastrand::Rng::new();
    let bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(16).collect();

    bytes
}

fn gen_iv() -> &'static [u8] {
    let iv = b"\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x33\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x33\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x33";

    iv
}

fn enc_dec() {
    let cipher = Cipher::aes_128_cbc();

    let original = String::from("Some secret plain Text to encrypt");
    println!("original: {}", original);

    let data = original.as_bytes();

    // this has to be 16 bytes
    let key = gen_key();

    // this can be any length
    let iv = gen_iv();

    let ciphertext = encrypt(cipher, &key, Some(iv), data).unwrap();
    println!("{:?}", ciphertext);

    let plaintext = decrypt(cipher, &key, Some(iv), &ciphertext[..]).unwrap();
    let decrypt_text = String::from_utf8(plaintext).unwrap();

    println!("{:?}", decrypt_text);

    assert_eq!(original, decrypt_text);
}

fn main() {
    /*
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
    */

    enc_dec();
}
