use anyhow::Result;
use age::secrecy::Secret;
use std::io::{Read, Write};

pub fn show_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("utf8 error: {}", e),
    }
}

fn main() -> Result<()> {

    let text = "this is a test string to be encrypted then decrypted.  it will first be converted to bytes using 'as_bytest'.".to_string();
    let plaintext = text.as_bytes();
    let passphrase = "a31f95aae7c3016156ea2b768d3c3d7dc382578a16e63d5b538ae066fadb20cb80e2800c5df493fe05bd2fd47369047906ab540addd442cc2872feb8c14b8418";

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

