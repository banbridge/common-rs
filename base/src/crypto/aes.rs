use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

use crate::error::{AppErrorBuilt, AppResult};

pub struct AesGcmCipher {
    cipher: Aes256Gcm,
}

impl AesGcmCipher {
    pub fn new(key: &[u8]) -> AppResult<Self> {
        let key = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Ok(Self { cipher })
    }

    pub fn new_from_base64(key_b64: &str) -> AppResult<Self> {
        let key = general_purpose::STANDARD
            .decode(key_b64)
            .map_err(|err| AppErrorBuilt::aes_decrypt_failed(err.to_string().into()))?;
        Self::new(&key)
    }

    pub fn encrypt_to_base64(&self, plaintext: &[u8]) -> AppResult<String> {
        let ciphertext = self.encrypt(plaintext)?;
        Ok(general_purpose::STANDARD.encode(ciphertext))
    }

    pub fn decrypt_from_base64(&self, ciphertext_b64: &str) -> AppResult<Vec<u8>> {
        let ciphertext = general_purpose::STANDARD
            .decode(ciphertext_b64)
            .map_err(|err| AppErrorBuilt::aes_decrypt_failed(err.to_string().into()))?;
        self.decrypt(&ciphertext)
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> AppResult<Vec<u8>> {
        let mut nonce = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce);
        let nonce = GenericArray::from_slice(&nonce);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|err| AppErrorBuilt::aes_encrypt_failed(err.to_string().into()))?;

        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(nonce);
        result.extend(ciphertext);

        Ok(result)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> AppResult<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(AppErrorBuilt::aes_decrypt_failed(
                "ciphertext too short".into(),
            ));
        }

        let nonce = GenericArray::from_slice(&ciphertext[..12]);
        let ciphertext = &ciphertext[12..];

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|err| AppErrorBuilt::aes_decrypt_failed(err.to_string().into()))?;

        Ok(plaintext)
    }
}

pub fn generate_aes256_key_base64() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    general_purpose::STANDARD.encode(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_encrypt_decrypt() {
        let key_b64 = generate_aes256_key_base64();
        let cipher = AesGcmCipher::new_from_base64(&key_b64).unwrap();

        let plaintext = b"hello world";
        let ciphertext_b64 = cipher.encrypt_to_base64(plaintext).unwrap();

        let decrypted = cipher.decrypt_from_base64(&ciphertext_b64).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_gcm_different_nonce() {
        let key_b64 = generate_aes256_key_base64();
        let cipher = AesGcmCipher::new_from_base64(&key_b64).unwrap();

        let plaintext = b"hello world";
        let c1 = cipher.encrypt_to_base64(plaintext).unwrap();
        let c2 = cipher.encrypt_to_base64(plaintext).unwrap();

        assert_ne!(c1, c2, "每次加密结果应该不同（nonce 随机）");

        let d1 = cipher.decrypt_from_base64(&c1).unwrap();
        let d2 = cipher.decrypt_from_base64(&c2).unwrap();
        assert_eq!(d1, d2);
        assert_eq!(d1, plaintext);
    }
}
