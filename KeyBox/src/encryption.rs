use aes_gcm::{
    Aes256Gcm, Key, Nonce, aead::{Aead, AeadCore, KeyInit, OsRng}
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use crate::errors::AppError;

pub struct Encryption {
    cipher: Aes256Gcm,
}

const NONCE_SIZE: usize = 12;

impl Encryption {
    pub fn new(key_str: &str) -> Encryption {
        let key = Key::<Aes256Gcm>::from_slice(key_str.as_ref());
        let cipher = Aes256Gcm::new(&key);
        Encryption { 
            cipher: cipher
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, AppError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_ref())?;

        let mut data = nonce.to_vec();
        data.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(&data))
    }

    pub fn decrypt(&self, encoded_data: &str) -> Result<String, AppError> {
        let data = STANDARD
            .decode(encoded_data)?;

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)?;

        Ok(String::from_utf8(plaintext)?)
    }
}
