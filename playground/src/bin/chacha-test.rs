
use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};

fn main() -> Result<()> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let text = "my plain text message for the chacha group";
    let ptext = text.as_bytes();
    let ciphertext = cipher.encrypt(&nonce, ptext.as_ref()).unwrap();

    println!("original.: {}", text);

    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();

    println!("decrypted: {}", String::from_utf8(plaintext)?);

    Ok(())

}