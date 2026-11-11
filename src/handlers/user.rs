use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthUser;
use crate::models::UserResponse;

/// Get current authenticated user
/// GET /api/users/me
/// Requires: Authorization: Bearer <token>
pub async fn get_current_user(
    auth_user: AuthUser,
    State(pool): State<PgPool>,
) -> AppResult<Json<UserResponse>> {
    // Query database for user
    let user: crate::models::User = sqlx::query_as(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1"
    )
        .bind(auth_user.user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::UserNotFound)?;

    let user_response: UserResponse = user.into();
    Ok(Json(user_response))
}
