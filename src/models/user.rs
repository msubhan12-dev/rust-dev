use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// User model - database representation
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User response - sent to client (no password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Register request - from client
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 1, message = "Confirm password is required"))]
    pub confirm_password: String,
}

impl RegisterRequest {
    /// Validate passwords match
    pub fn validate_passwords_match(&self) -> Result<(), String> {
        if self.password != self.confirm_password {
            return Err("Passwords do not match".to_string());
        }
        Ok(())
    }
}

/// Login request - from client
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Login response - sent to client with JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
    pub expires_in: i64, // seconds
}

/// Register response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
    pub message: String,
}

/// Refresh token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String,
}

/// Refresh token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub expires_in: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_validation() {
        let req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "SecurePassword123".to_string(),
            confirm_password: "SecurePassword123".to_string(),
        };

        assert!(req.validate().is_ok());
        assert!(req.validate_passwords_match().is_ok());
    }

    #[test]
    fn test_register_password_mismatch() {
        let req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "SecurePassword123".to_string(),
            confirm_password: "DifferentPassword".to_string(),
        };

        assert!(req.validate_passwords_match().is_err());
    }

    #[test]
    fn test_register_invalid_email() {
        let req = RegisterRequest {
            email: "not-an-email".to_string(),
            password: "SecurePassword123".to_string(),
            confirm_password: "SecurePassword123".to_string(),
        };

        assert!(req.validate().is_err());
    }

    #[test]
    fn test_register_password_too_short() {
        let req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            confirm_password: "short".to_string(),
        };

        assert!(req.validate().is_err());
    }

    #[test]
    fn test_user_to_response() {
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response: UserResponse = user.into();
        assert_eq!(response.email, "test@example.com");
        assert!(response.password_hash.is_empty() || !response.password_hash.contains("hashed"));
    }
}
