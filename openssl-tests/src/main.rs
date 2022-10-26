// use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};

fn enc_dec() {
    let cipher = Cipher::aes_128_cbc();

    let original = String::from("Some secret plain Text to encrypt");
    println!("original: {}", original);

    let data = original.as_bytes();

    // let key = b"
    // let iv = b"\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let key = b"\x0F\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let iv = b"\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x0F\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let ciphertext = encrypt(cipher, key, Some(iv), data).unwrap();
    println!("{:?}", ciphertext);

    let plaintext = decrypt(cipher, key, Some(iv), &ciphertext[..]).unwrap();
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
