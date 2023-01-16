
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

pub fn encrypt(cck: ChaChaKeys, text: &str) -> Result<Vec<u8>> {

    let ptext = text.as_bytes();
    let cipher = ChaCha20Poly1305::new(&cck.key);
    let ciphertext = cipher.encrypt(&cck.nonce, ptext.as_ref()).unwrap();

    Ok(ciphertext)
}

pub fn decrypt(cck: ChaChaKeys, ciphertext: Vec<u8>) -> Result<String> {
    let text = String::new();

    Ok(text)
}

fn main() -> Result<()> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let cck = ChaChaKeys {
        key,
        nonce,
    };

    let json = serde_json::to_string(&cck)?;
    println!("{}", json);

    let text = "my plain text message for the chacha group";
    let ciphertext = encrypt(cck, text)?;
    println!("{:?}", ciphertext);

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