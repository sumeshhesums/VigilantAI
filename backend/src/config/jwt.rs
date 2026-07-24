use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// PEM-encoded RSA private key for signing tokens.
    pub private_key: String,
    /// PEM-encoded RSA public key for verifying tokens.
    pub public_key: String,
    /// Access token lifetime in seconds (default: 900 = 15 minutes).
    pub access_token_expiry_secs: u64,
    /// Refresh token lifetime in seconds (default: 604800 = 7 days).
    pub refresh_token_expiry_secs: u64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self> {
        let private_key = std::env::var("JWT_PRIVATE_KEY")
            .context("JWT_PRIVATE_KEY environment variable not set")?;

        let public_key = std::env::var("JWT_PUBLIC_KEY")
            .context("JWT_PUBLIC_KEY environment variable not set")?;

        let access_token_expiry_secs = std::env::var("JWT_ACCESS_TOKEN_EXPIRY_SECS")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<u64>()
            .context("JWT_ACCESS_TOKEN_EXPIRY_SECS must be a valid u64")?;

        let refresh_token_expiry_secs = std::env::var("JWT_REFRESH_TOKEN_EXPIRY_SECS")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<u64>()
            .context("JWT_REFRESH_TOKEN_EXPIRY_SECS must be a valid u64")?;

        Ok(Self {
            private_key,
            public_key,
            access_token_expiry_secs,
            refresh_token_expiry_secs,
        })
    }
}
