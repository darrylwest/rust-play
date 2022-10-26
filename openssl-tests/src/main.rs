use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::base64::encode_block;
use std::iter::repeat_with;

fn gen_key() -> Vec<u8> {
    let rng = fastrand::Rng::new();
    let bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(16).collect();

    bytes
}

fn gen_iv(size: u8) -> Vec<u8> {
    let sz = size as usize;
    let rng = fastrand::Rng::new();
    let iv: Vec<u8> = repeat_with(|| rng.u8(..)).take(sz).collect();

    iv
}

fn save_key(key: &Vec<u8>, iv: &Vec<u8>) {
    let sz = key.len() + iv.len();
    let mut v = Vec::with_capacity(sz);

    for n in key {
        v.push(*n);
    }

    for n in iv {
        v.push(*n);
    }

    let b64 = encode_block(v.as_slice());
    println!("{:?}", &v);
    println!("{}", b64);

    // v
}

fn enc_dec() {
    let cipher = Cipher::aes_128_cbc();

    let original = String::from("Some secret plain Text to encrypt");
    println!("original: {}", original);

    let data = original.as_bytes();

    // this has to be 16 bytes
    let key = gen_key();

    // this can be any length
    let iv_size: u8 = fastrand::u8(16..64);
    // println!("iv size: {}", iv_size);
    let iv = gen_iv(iv_size);

    save_key(&key, &iv);

    let ciphertext = encrypt(cipher, &key, Some(&iv), data).unwrap();
    println!("{:?}", ciphertext);

    let plaintext = decrypt(cipher, &key, Some(&iv), &ciphertext[..]).unwrap();
    let decrypt_text = String::from_utf8(plaintext).unwrap();

    println!("{:?}", decrypt_text);

    assert_eq!(original, decrypt_text);
}

pub fn check_version() {
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
}

fn main() {

    enc_dec();
}
