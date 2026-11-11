use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::JwtHandler;
use crate::error::{AppError, AppResult};
use crate::models::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserResponse};
use crate::password;

/// Register new user
/// POST /api/auth/register
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<RegisterResponse>)> {
    // Validate request
    payload.validate()
        .map_err(|_| AppError::InvalidPassword)?;
    
    payload.validate_passwords_match()
        .map_err(|_| AppError::InvalidPassword)?;

    // Check if email already exists
    let existing_user: Option<(String,)> = sqlx::query_as(
        "SELECT email FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if existing_user.is_some() {
        return Err(AppError::EmailAlreadyExists);
    }

    // Hash password
    let password_hash = password::hash_password(&payload.password)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Create user in database
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user: crate::models::User = sqlx::query_as(
        "INSERT INTO users (id, email, password_hash, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, email, password_hash, created_at, updated_at"
    )
        .bind(user_id)
        .bind(&payload.email)
        .bind(&password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let user_response: UserResponse = user.into();

    let response = RegisterResponse {
        user: user_response,
        message: "User registered successfully".to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Login user
/// POST /api/auth/login
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    // Validate request
    payload.validate()
        .map_err(|_| AppError::InvalidEmail)?;

    // Find user by email
    let user: crate::models::User = sqlx::query_as(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let is_password_valid = password::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    if !is_password_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Generate JWT token
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET not found");
    let jwt_handler = JwtHandler::new(jwt_secret);

    let token = jwt_handler
        .generate_token(user.id, user.email.clone())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let user_response: UserResponse = user.into();

    let response = LoginResponse {
        user: user_response,
        token,
        expires_in: 86400, // 24 hours in seconds
    };

    Ok(Json(response))
}

/// Logout user (client-side implementation)
/// POST /api/auth/logout
pub async fn logout() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}
