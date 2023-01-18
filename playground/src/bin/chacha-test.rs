use anyhow::Result;

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

fn main() -> Result<()> {
    let text = "my plain text message for the chacha group";
    let blob = text.as_bytes();
    println!("original: {}", text);

    let cck = chacha::generate_keys();

    let json = serde_json::to_string(&cck)?;
    println!("{}", json);

    let ciphertext = chacha::encrypt(&cck, blob)?;
    println!("{:?}", ciphertext);

    let vtext = chacha::decrypt(&cck, ciphertext)?;
    let stext = String::from_utf8(vtext)?;
    println!("decrypted: {}", stext);

    assert_eq!(text, stext);

    Ok(())
}
