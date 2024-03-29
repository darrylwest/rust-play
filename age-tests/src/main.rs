use anyhow::Result;
use age::secrecy::Secret;
use std::io::{Read, Write};
// use std::fs;
// use std::path::PathBuf;
use std::iter;
use age::x25519::Identity;
use age::stream::StreamWriter;

// asymmetric keys
fn main() -> Result<()> {
    ssh()
}

pub fn ssh() -> Result<()> {
    let key = Identity::generate();
    println!("key: {:?}", key.to_string());

    let pubkey = key.to_public();
    println!("pub: {:?}", pubkey.to_string());

    let string_key = pubkey.to_string();

    // now send the public key to remote service

    let plaintext = "this is a test text blob".as_bytes();

    let encrypted = encrypt(&string_key, plaintext);


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

pub fn encrypt<W>(pubkey: &str, text: &[u8]) -> Result<StreamWriter<W>> {
    let encryptor = age::Encryptor::with_recipients(vec![Box::new(pubkey)]).expect("a recipient");
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(text)?;
    writer.finish()?;

    println!("enc: {:?}", encrypted);

    encrypted
}

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

pub fn read_passphrase() -> Result<String> {
    let mut buf = String::new();

    println!("Enter pass phrase: ");
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;

    Ok(buf)
}

// symmetric keys
pub fn password_encoder() -> Result<()> {
    let text = "this is a test string to be encrypted then decrypted.  it will first be converted to bytes using 'as_bytes'.".to_string();
    let plaintext = text.as_bytes();
    let passphrase = "a31f95aae7c3016156ea2b768d3c3d7dc382578a16e63d5b538ae066fadb20cb80e2800c5df493fe05bd2fd47369047906ab540addd442cc2872feb8c14b8418";
    //
    // prompt for the pass phrase
    // let passphrase = read_passphrase()?;
    //
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
