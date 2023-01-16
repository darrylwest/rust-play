
use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use generic_array::GenericArray;
use typenum::{U12, U32};
use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChaChaKeys {
    pub key: GenericArray<u8, U32>,
    pub nonce: GenericArray<u8, U12>,
}

pub fn encrypt(cck: &ChaChaKeys, text: &str) -> Result<Vec<u8>> {

    let ptext = text.as_bytes();
    let cipher = ChaCha20Poly1305::new(&cck.key);
    let ciphertext = cipher.encrypt(&cck.nonce, ptext.as_ref()).unwrap();

    Ok(ciphertext)
}

pub fn decrypt(cck: &ChaChaKeys, ciphertext: Vec<u8>) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(&cck.key);
    let vtext = cipher.decrypt(&cck.nonce, ciphertext.as_ref()).unwrap();

    Ok(vtext)
}

fn main() -> Result<()> {
    let text = "my plain text message for the chacha group";
    println!("original: {}", text);

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let cck = ChaChaKeys {
        key,
        nonce,
    };

    let json = serde_json::to_string(&cck)?;
    println!("{}", json);


    let ciphertext = encrypt(&cck, text)?;
    println!("{:?}", ciphertext);

    let vtext = decrypt(&cck, ciphertext)?;
    let stext = String::from_utf8(vtext)?;
    println!("decrypted: {}", stext);

    assert_eq!(text, stext);

    /*

    let cipher = ChaCha20Poly1305::new(&cck.key);
    let ciphertext: Vec<u8> = cipher.encrypt(&cck.nonce, ptext.as_ref()).expect("should encrypt");
    println!("{:?}", ciphertext);


    println!("original.: {}", text);

    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    println!("decrypted: {}", String::from_utf8(plaintext)?);
     */

    Ok(())

}