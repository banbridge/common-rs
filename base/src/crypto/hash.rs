use base64::{engine::general_purpose, Engine as _};
use hex::ToHex;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Sha256, Sha512};

use crate::error::{AppErrorBuilt, AppResult};

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
type HmacMd5 = Hmac<Md5>;

pub fn md5_hex(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    hasher.finalize().encode_hex::<String>()
}

pub fn md5_base64(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().encode_hex::<String>()
}

pub fn sha256_base64(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn sha512_hex(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    hasher.finalize().encode_hex::<String>()
}

pub fn sha512_base64(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn hmac_sha256_hex(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(mac.finalize().into_bytes().encode_hex::<String>())
}

pub fn hmac_sha256_base64(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

pub fn hmac_sha512_hex(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacSha512::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(mac.finalize().into_bytes().encode_hex::<String>())
}

pub fn hmac_sha512_base64(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacSha512::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

pub fn hmac_md5_hex(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacMd5::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(mac.finalize().into_bytes().encode_hex::<String>())
}

pub fn hmac_md5_base64(key: &[u8], input: &str) -> AppResult<String> {
    let mut mac = HmacMd5::new_from_slice(key)
        .map_err(|err| AppErrorBuilt::crypto_error(err.to_string().into()))?;
    mac.update(input.as_bytes());
    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hex() {
        let result = md5_hex("hello");
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha256_hex() {
        let result = sha256_hex("hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hmac_sha256_hex() {
        let key = b"secret";
        let result = hmac_sha256_hex(key, "hello").unwrap();
        assert_eq!(
            result,
            "88aab3ede8d3adf94d26ab90d3baf3481316a766697934d2d9435702e1c3a7cb"
        );
    }
}
