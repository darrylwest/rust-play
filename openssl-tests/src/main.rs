use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::base64::encode_block;
use std::iter::repeat_with;
use openssl::sha;
// use std::io::prelude::*;
// use std::fs::File;

// key is a fixed 32 bytes for 256
pub fn gen_key() -> Vec<u8> {
    let rng = fastrand::Rng::new();
    let bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(32).collect();

    bytes
}

// iv is any size requested but 64 is a good guess
pub fn gen_iv(size: u8) -> Vec<u8> {
    println!("request size: {}", size);
    let sz = size as usize;
    let rng = fastrand::Rng::new();
    let iv: Vec<u8> = repeat_with(|| rng.u8(..)).take(sz).collect();

    iv
}

pub fn save_key(key: &Vec<u8>, iv: &Vec<u8>) {
    let sz = key.len() + iv.len();
    let mut v = Vec::with_capacity(sz);

    for n in key {
        v.push(*n);
    }

    // v.push(b".");

    for n in iv {
        v.push(*n);
    }

    let b64 = encode_block(v.as_slice());
    // println!("{:?}", &v);
    println!("key/iv: {}", b64);

    // v
}

pub fn save_encoded(data: &Vec<u8>) {
    let b64 = encode_block(data.as_slice());

    println!("data {}", b64);
}

pub fn enc_dec() {
    let cipher = Cipher::aes_256_cbc();

    let original = String::from("Some secret plain Text to encrypt");
    println!("original: {}", original);

    let data = original.as_bytes();

    // this has to be 32 bytes
    let key = gen_key();
    println!("key length: {}", key.len());

    // this can be any length
    let iv_size: u8 = 32; // fastrand::u8(32..64);
    // println!("iv size: {}", iv_size);
    let iv = gen_iv(iv_size);
    println!("iv length: {}", iv.len());

    save_key(&key, &iv);

    let ciphertext = encrypt(cipher, &key, Some(&iv), data).unwrap();
    save_encoded(&ciphertext);

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

pub fn hash() {
    let mut hasher = sha::Sha256::new();

    let content = std::fs::read("src/main.rs").unwrap();

    hasher.update(content.as_slice());

    let hash = hasher.finish();

    println!("hash: {}", hex::encode(hash));
}

pub fn ed25519() {
    use openssl::pkey::{PKey, Id};

    println!("generate new priv/pub keys...");

    let key = PKey::generate_ed25519().expect("should generate the private key");
    let public_key = key.raw_public_key().expect("should create the pub side");

    println!("priv: {:?}", key);
    println!("pub: {:?}", public_key);

}

pub fn ssl() {
    use openssl::ssl::{SslMethod, SslConnector};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("github.com:443").unwrap();
    let mut stream = connector.connect("github.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut resp = vec![];
    stream.read_to_end(&mut resp).unwrap();

    println!("{}", String::from_utf8_lossy(&resp));
}

fn main() {
    // check_version();
    // enc_dec();
    // hash();
    ed25519();
    // ssl();
}
