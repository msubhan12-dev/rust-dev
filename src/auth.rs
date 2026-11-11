use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // User UUID
    pub email: String,
    pub iat: i64,          // Issued at
    pub exp: i64,          // Expiration time
}

impl Claims {
    /// Create new JWT claims
    pub fn new(user_id: Uuid, email: String) -> Self {
        let now = Utc::now();
        let exp = (now + chrono::Duration::hours(24)).timestamp();

        Self {
            sub: user_id.to_string(),
            email,
            iat: now.timestamp(),
            exp,
        }
    }
}

/// JWT Handler
pub struct JwtHandler {
    secret: String,
}

impl JwtHandler {
    /// Create new JWT handler
    pub fn new(secret: String) -> Self {
        if secret.is_empty() {
            eprintln!("⚠️  WARNING: JWT_SECRET is empty! Using default (NOT FOR PRODUCTION)");
        }
        Self { secret }
    }

    /// Generate JWT token
    pub fn generate_token(&self, user_id: Uuid, email: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, email);
        let key = EncodingKey::from_secret(self.secret.as_bytes());
        
        encode(&Header::default(), &claims, &key)
    }

    /// Validate and decode JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let data = decode::<Claims>(
            token,
            &key,
            &Validation::default(),
        )?;

        Ok(data.claims)
    }

    /// Extract token from Authorization header
    pub fn extract_token_from_header(auth_header: &str) -> Option<String> {
        auth_header
            .strip_prefix("Bearer ")
            .map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let jwt_handler = JwtHandler::new("test-secret-key".to_string());
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        // Generate token
        let token = jwt_handler
            .generate_token(user_id, email.clone())
            .expect("Failed to generate token");

        // Validate token
        let claims = jwt_handler
            .validate_token(&token)
            .expect("Failed to validate token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_invalid_token() {
        let jwt_handler = JwtHandler::new("test-secret-key".to_string());
        let result = jwt_handler.validate_token("invalid-token");

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token() {
        let header = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";
        let token = JwtHandler::extract_token_from_header(header);

        assert_eq!(
            token,
            Some("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9".to_string())
        );
    }
}
