use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// API Error Response
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Custom application error types
#[derive(Debug)]
pub enum AppError {
    // Authentication errors
    InvalidEmail,
    InvalidPassword,
    PasswordTooShort,
    EmailAlreadyExists,
    InvalidCredentials,
    TokenExpired,
    InvalidToken,
    TokenMissing,

    // Database errors
    DatabaseError(String),
    UserNotFound,

    // Server errors
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidEmail => write!(f, "Invalid email format"),
            AppError::InvalidPassword => write!(f, "Invalid password format"),
            AppError::PasswordTooShort => write!(f, "Password must be at least 8 characters"),
            AppError::EmailAlreadyExists => write!(f, "Email already registered"),
            AppError::InvalidCredentials => write!(f, "Invalid email or password"),
            AppError::TokenExpired => write!(f, "Token has expired"),
            AppError::InvalidToken => write!(f, "Invalid token"),
            AppError::TokenMissing => write!(f, "Authorization token missing"),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::UserNotFound => write!(f, "User not found"),
            AppError::InternalServerError(e) => write!(f, "Internal server error: {}", e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type) = match self {
            AppError::InvalidEmail => (StatusCode::BAD_REQUEST, "INVALID_EMAIL"),
            AppError::InvalidPassword => (StatusCode::BAD_REQUEST, "INVALID_PASSWORD"),
            AppError::PasswordTooShort => (StatusCode::BAD_REQUEST, "PASSWORD_TOO_SHORT"),
            AppError::EmailAlreadyExists => (StatusCode::CONFLICT, "EMAIL_EXISTS"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED"),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN"),
            AppError::TokenMissing => (StatusCode::UNAUTHORIZED, "TOKEN_MISSING"),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            AppError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

/// Result type using AppError
pub type AppResult<T> = Result<T, AppError>;
