use faststr::FastStr;

use crate::error::{AppErrorBuilt, AppResult};

#[allow(unused)]

pub struct BcryptEncoder;

impl BcryptEncoder {
    pub fn encode(&self, password: FastStr) -> AppResult<FastStr> {
        let hash_paas = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|err| AppErrorBuilt::bcrypt_failed(err.to_string().into()))?;

        Ok(hash_paas.into())
    }

    pub fn matches(&self, raw_password: FastStr, encoded_password: FastStr) -> AppResult<bool> {
        bcrypt::verify(raw_password, encoded_password.as_str())
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

        let result = BcryptEncoder.matches(password, hash_password);

        assert!(result.is_ok());

        println!("result: {:?}", result);
    }
}
