use faststr::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Claim<T> {
    //pub jti: String,    // jwt id
    pub exp: u64, // 过期时间
    pub iat: u64, // 签发时间
    pub iss: FastStr,
    pub user: T, // 用户名
}
