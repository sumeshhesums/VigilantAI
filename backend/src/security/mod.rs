pub mod jwt;
pub mod password;

use anyhow::{Context, Result};
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::config::JwtConfig;

#[derive(Clone)]
pub struct Security {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub access_token_expiry_secs: u64,
    pub refresh_token_expiry_secs: u64,
}

impl Security {
    pub fn from_config(config: &JwtConfig) -> Result<Self> {
        let encoding_key = EncodingKey::from_rsa_pem(config.private_key.as_bytes())
            .context("invalid RSA private key")?;

        let decoding_key = DecodingKey::from_rsa_pem(config.public_key.as_bytes())
            .context("invalid RSA public key")?;

        Ok(Self {
            encoding_key,
            decoding_key,
            access_token_expiry_secs: config.access_token_expiry_secs,
            refresh_token_expiry_secs: config.refresh_token_expiry_secs,
        })
    }
}
