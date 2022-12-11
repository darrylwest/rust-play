use anyhow::Result;
use age::secrecy::Secret;
use std::io::{Read, Write};
// use std::fs;
// use std::path::PathBuf;
use std::iter;
// use age::x25519::Identity;
// use std::str::FromStr;

    /*
    let home = std::env::var("HOME")?;
    let pkey_file: PathBuf = [&home, ".ssh/ed25519"].iter().collect();
    let key_file: PathBuf = [ &home, ".ssh/key-service.key.enc"].iter().collect();

    println!("{} {}", pkey_file.to_str().unwrap(), key_file.to_str().unwrap());

    let pkey = fs::read_to_string(pkey_file.to_str().unwrap())?;
    let key = Identity::from_str(&pkey).unwrap();

    let bytes = fs::read(key_file.to_str().unwrap())?;
    println!("{:?}", bytes);
    */

fn main() -> Result<()> {
    let key = age::x25519::Identity::generate();
    // println!("key: {:?}", key.to_string());

    let pubkey = key.to_public();
    println!("pub: {:?}", pubkey.to_string());

    let plaintext = b"this is a test text blob";

    let encrypted = {
        let encryptor = age::Encryptor::with_recipients(vec![Box::new(pubkey)]).expect("a recipient");
        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };

    println!("enc: {:?}", encrypted);

    let decrypted = {
        let decryptor = match age::Decryptor::new(&encrypted[..])? {
            age::Decryptor::Recipients(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(iter::once(&key as &dyn age::Identity))?;
        reader.read_to_end(&mut decrypted)?;

        decrypted
    };

    assert_eq!(decrypted, plaintext);
    show_utf8(decrypted);

    Ok(())
}

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

pub fn read_passphrase() -> Result<String> {
    let mut buf = String::new();

    println!("Enter passs phrase: ");
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;

    Ok(buf)
}

pub fn password_encoder() -> Result<()> {
    let text = "this is a test string to be encrypted then decrypted.  it will first be converted to bytes using 'as_bytes'.".to_string();
    let plaintext = text.as_bytes();
    // let passphrase = "a31f95aae7c3016156ea2b768d3c3d7dc382578a16e63d5b538ae066fadb20cb80e2800c5df493fe05bd2fd47369047906ab540addd442cc2872feb8c14b8418";
    //
    // prompt for the pass phrase
    let passphrase = read_passphrase()?;
    println!("ok, working...");

    // Encrypt the plaintext to a ciphertext using the passphrase...
    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };


    // ... and decrypt the ciphertext to the plaintext again using the same passphrase.
    println!("and, working...");
    let decrypted = {
        let decryptor = match age::Decryptor::new(&encrypted[..])? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None)?;
        reader.read_to_end(&mut decrypted)?;

        decrypted
    };

    show_utf8(plaintext.to_vec());
    println!("{:?}", encrypted);
    show_utf8(decrypted);

    Ok(())
}

