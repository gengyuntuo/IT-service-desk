// src/user
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

// 从环境变量读取密钥（生产必备！）
fn secret_key() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "my-super-secret-key-123!".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    // Standard claims
    pub sub: String, // user ID
    pub exp: usize,  // expiration (Unix timestamp)
    pub iat: usize,  // issued at

    // Custom claims
    pub role: String,
}

impl AuthClaims {
    pub fn new(user_id: String, role: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Self {
            sub: user_id,
            iat: now,
            exp: now + 24 * 60 * 60, // 24 hours
            role,
        }
    }
}

pub fn create_token(claims: &AuthClaims) -> Result<String, jsonwebtoken::errors::Error> {
    let key = EncodingKey::from_secret(secret_key().as_bytes());
    encode(&Header::new(Algorithm::HS256), claims, &key)
}

pub fn validate_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret_key().as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<AuthClaims>(token, &key, &validation)?;
    Ok(token_data.claims)
}
