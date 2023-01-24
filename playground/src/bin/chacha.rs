use anyhow::Result;
use clap::Parser;
use std::{fs::File, io::{Read, Write}};
use std::path::Path;


pub mod chacha {
    use anyhow::{anyhow, Result};
    use chacha20poly1305::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        ChaCha20Poly1305,
    };
    use generic_array::GenericArray;
    use serde::{Deserialize, Serialize};
    use typenum::{U12, U32};

    /// generate the key/nonce
    pub fn generate_keys() -> Keys {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

        Keys { key, nonce }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Keys {
        pub key: GenericArray<u8, U32>,
        pub nonce: GenericArray<u8, U12>,
    }

    /// encrypt the byte-blob using ChaCha20Poly1305
    pub fn encrypt(cck: &Keys, blob: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(&cck.key);
        if let Ok(ciphertext) = cipher.encrypt(&cck.nonce, blob.as_ref()) {
            Ok(ciphertext)
        } else {
            Err(anyhow!("broken cha cha encrypt"))
        }
    }

    /// decrypt the cipher text using ChaCha20Poly1305
    pub fn decrypt(cck: &Keys, ciphertext: Vec<u8>) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(&cck.key);

        if let Ok(vtext) = cipher.decrypt(&cck.nonce, ciphertext.as_ref()) {
            Ok(vtext)
        } else {
            Err(anyhow!("broken cha cha decrypt"))
        }
    }
}

#[derive(Debug, Default, Parser)]
struct Cli {
    /// name the plain text file
    #[clap(short, long, value_parser)]
    plain: String,
    /// name the cipher text file
    #[clap(short, long, value_parser)]
    cipher: String,
    /// name the key/nonce file
    #[clap(short, long, value_parser)]
    key_file: String,
    /// set true to encrypt
    #[clap(short, long, value_parser)]
    encrypt: bool,
    /// set true to decrypt
    #[clap(short, long, value_parser)]
    decrypt: bool,
}

impl Cli {
    fn new() -> Cli {
        Cli::parse()
    }
}

fn read_file(filename: String) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf: Vec<u8> = vec![];

    file.read_to_end(&mut buf)?;

    Ok(buf)
}

fn write_file(filename: String, buf: Vec<u8>) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(&buf)?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::new();

    println!("{:?}", cli);

    if cli.encrypt {
        // if keys file exists, then read and use it
        let cck = if Path::exists(Path::new(&cli.key_file)) {
            println!("read existing key from {}", &cli.key_file);
            let json = read_file(cli.key_file)?;
            serde_json::from_slice(&json)?
        } else {
            println!("generate new keys");
            let keys = chacha::generate_keys();

            let json = serde_json::to_string(&keys)?;
            // write the keys to file
            write_file(cli.key_file.clone(), json.into_bytes())?;
            println!("saved keys as json to: {}", cli.key_file);

            keys
        };


        // read the input file
        let blob = read_file(cli.plain)?;
        println!("blob size: {}", blob.len());

        // encrypt
        let ciphertext = chacha::encrypt(&cck, &blob)?;
        write_file(cli.cipher.clone(), ciphertext)?;
        println!("encrypted file saved to {}", cli.cipher);
    } else {
        println!("read existing key from {}", &cli.key_file);
        let json = read_file(cli.key_file)?;
        let cck = serde_json::from_slice(&json)?;

        let ciphertext = read_file(cli.cipher)?;
        let vtext = chacha::decrypt(&cck, ciphertext)?;

        write_file(cli.plain.clone(), vtext)?;
        println!("decrypted text written to : {}", &cli.plain);
    }


    Ok(())
}
