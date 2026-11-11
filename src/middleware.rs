use std::future::Future;
use std::pin::Pin;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::auth::JwtHandler;
use crate::error::AppError;

/// JWT Claims extractor for protected routes
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

/// Extract JWT claims from Authorization header
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts<'a>(
        parts: &'a mut Parts,
        _state: &S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'a>> {
        Box::pin(async move {
            // Extract Authorization header
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or(AppError::TokenMissing)?;

            // Extract Bearer token
            let token = JwtHandler::extract_token_from_header(auth_header)
                .ok_or(AppError::TokenMissing)?;

            // Get JWT secret from environment
            let jwt_secret = std::env::var("JWT_SECRET")
                .map_err(|_| AppError::InternalServerError("JWT_SECRET not configured".to_string()))?;

            // Initialize JWT handler
            let jwt_handler = JwtHandler::new(jwt_secret);

            // Validate and decode token
            let claims = jwt_handler
                .validate_token(&token)
                .map_err(|e| {
                    if e.to_string().contains("ExpiredSignature") {
                        AppError::TokenExpired
                    } else {
                        AppError::InvalidToken
                    }
                })?;

            // Parse user_id from claims
            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| AppError::InvalidToken)?;

            Ok(AuthUser {
                user_id,
                email: claims.email,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_creation() {
        let auth_user = AuthUser {
            user_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
        };

        assert!(!auth_user.email.is_empty());
        assert!(auth_user.user_id.is_nil() == false);
    }
}
