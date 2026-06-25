use faststr::FastStr;

use crate::error::{AppErrorBuilt, AppResult};

pub struct BcryptEncoder;

impl BcryptEncoder {
    pub fn encode<P: AsRef<[u8]>>(&self, password: P) -> AppResult<String> {
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|err| AppErrorBuilt::bcrypt_failed(err.to_string().into()))?;

        Ok(hash.into())
    }

    pub fn matches<P: AsRef<[u8]>>(
        &self,
        raw_password: P,
        encoded_password: &str,
    ) -> AppResult<bool> {
        bcrypt::verify(raw_password, encoded_password)
            .map_err(|err| AppErrorBuilt::bcrypt_failed(err.to_string().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = FastStr::from("12345678");

        let hash_password = BcryptEncoder.encode(password.clone()).unwrap_or_default();

        println!("hash_password: {}", hash_password);

        let result = BcryptEncoder.matches(password, hash_password.as_str());

        assert!(result.is_ok());

        println!("result: {:?}", result);
    }
}
