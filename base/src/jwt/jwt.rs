use std::time::Duration;

use faststr::FastStr;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    error::{AppErrorBuilt, AppResult},
    jwt::Claim,
};

#[derive(Clone, Debug)]

pub struct JWTManager {
    encode_key: EncodingKey,
    decode_key: DecodingKey,

    header: Header,
    validation: Validation,
    expiration: Duration,
    issuer: FastStr,
}

impl JWTManager {
    pub fn new(encode_key: &str, expiration_second: u64, issuer: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.set_audience(&["me"]);

        validation.set_required_spec_claims(&["exp", "user", "iat"]);

        Self {
            encode_key: EncodingKey::from_secret(encode_key.as_bytes()),
            decode_key: DecodingKey::from_secret(encode_key.as_bytes()),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: Duration::from_secs(expiration_second),
            issuer: FastStr::from_string(issuer.to_string()),
        }
    }

    pub fn with_expiration(mut self, expiration_second: u64) -> Self {
        self.expiration = Duration::from_secs(expiration_second);

        self
    }

    pub fn encode<T>(&self, principal: T) -> AppResult<String>
    where
        T: Serialize + Clone + DeserializeOwned,
    {
        let current_time = get_current_timestamp();

        let claim = Claim {
            exp: current_time.saturating_add(self.expiration.as_secs()),
            iat: current_time,
            user: principal,
            iss: self.issuer.clone(),
        };

        let jwt_token =
            jsonwebtoken::encode(&self.header, &claim, &self.encode_key).map_err(|err| {
                AppErrorBuilt::jwt_encode(format!("jwt encode err: {:?}", err).into())
            })?;

        Ok(jwt_token.into())
    }

    pub fn decode<T>(&self, token: &str) -> AppResult<T>
    where
        T: Serialize + Clone + DeserializeOwned,
    {
        let token_data =
            jsonwebtoken::decode::<Claim<T>>(token, &self.decode_key, &self.validation).map_err(
                |err| AppErrorBuilt::jwt_decode(format!("jwt decode err: {:?}", err).into()),
            )?;

        Ok(token_data.claims.user)
    }
}

#[cfg(test)]

mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone)]

    pub struct Principal {
        user_id: String,
        username: String,
    }

    #[test]

    fn test_jwt() {
        let jwt = JWTManager::new("secret", 3600, "admin");

        let p = Principal {
            user_id: "b@b.com".to_string(),
            username: "b@b.com".to_string(),
        };

        let token = jwt.encode(p).unwrap();

        println!("{}", token);

        let p = jwt.decode::<Principal>(&token);

        println!("{:?}", p);
    }
}
