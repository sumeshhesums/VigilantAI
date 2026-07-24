use anyhow::{Context, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID as string).
    pub sub: String,
    /// User email.
    pub email: String,
    /// User role.
    pub role: String,
    /// Expiration timestamp (Unix).
    pub exp: u64,
    /// Issued-at timestamp (Unix).
    pub iat: u64,
    /// Unique token identifier.
    pub jti: String,
}

/// Create an access token (short-lived, default 15 minutes).
pub fn create_access_token(
    user_id: uuid::Uuid,
    email: &str,
    role: &str,
    expiry_secs: u64,
    encoding_key: &EncodingKey,
) -> Result<String> {
    create_token(user_id, email, role, expiry_secs, encoding_key)
}

/// Create a refresh token (long-lived, default 7 days).
pub fn create_refresh_token(
    user_id: uuid::Uuid,
    email: &str,
    role: &str,
    expiry_secs: u64,
    encoding_key: &EncodingKey,
) -> Result<String> {
    create_token(user_id, email, role, expiry_secs, encoding_key)
}

fn create_token(
    user_id: uuid::Uuid,
    email: &str,
    role: &str,
    expiry_secs: u64,
    encoding_key: &EncodingKey,
) -> Result<String> {
    let now = Utc::now().timestamp() as u64;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: now + expiry_secs,
        iat: now,
        jti: uuid::Uuid::new_v4().to_string(),
    };

    let header = Header::new(Algorithm::RS256);

    encode(&header, &claims, encoding_key).context("failed to encode JWT")
}

/// Validate and decode a JWT, returning the claims.
pub fn validate_token(token: &str, decoding_key: &DecodingKey) -> Result<Claims> {
    let validation = Validation::new(Algorithm::RS256);

    let token_data =
        decode::<Claims>(token, decoding_key, &validation).context("failed to validate JWT")?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "test-user-id".to_string(),
            email: "test@example.com".to_string(),
            role: "admin".to_string(),
            exp: 1700000000,
            iat: 1699999100,
            jti: "token-uuid".to_string(),
        };

        let json = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(claims.sub, deserialized.sub);
        assert_eq!(claims.email, deserialized.email);
        assert_eq!(claims.role, deserialized.role);
        assert_eq!(claims.exp, deserialized.exp);
        assert_eq!(claims.iat, deserialized.iat);
        assert_eq!(claims.jti, deserialized.jti);
    }
}
