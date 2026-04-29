use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::Deserialize;
use sqlx::{MySql, Pool};

use super::{verify_password, generate_token, hash_token, get_session_expiry, AuthResponse, ErrorResponse, UserAgent};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[post("/auth/login", format = "json", data = "<req>")]
pub async fn login(
    pool: &State<Pool<MySql>>,
    user_agent: UserAgent,
    req: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (Status, Json<ErrorResponse>)> {
    // Query user by username or email
    let user: Option<(i32, String, String)> = sqlx::query_as(
        "SELECT user_id, username, password_hash FROM users WHERE username = ? OR email = ?"
    )
    .bind(&req.username_or_email)
    .bind(&req.username_or_email)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Database error".to_string(),
        }),
    ))?;

    let (user_id, username, password_hash) = user.ok_or((
        Status::Unauthorized,
        Json(ErrorResponse {
            error: "Invalid username/email or password".to_string(),
        }),
    ))?;

    // Verify password
    let password_valid = verify_password(&req.password, &password_hash).map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to verify password".to_string(),
        }),
    ))?;

    if !password_valid {
        return Err((
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "Invalid username/email or password".to_string(),
            }),
        ));
    }

    // Generate new session token
    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at = get_session_expiry();

    // Insert session into database
    sqlx::query(
        "INSERT INTO user_sessions (token_hash, user_id, expires_at, created_at, user_agent) VALUES (?, ?, ?, NOW(), ?)"
    )
    .bind(&token_hash)
    .bind(user_id)
    .bind(expires_at.to_rfc3339())
    .bind(&user_agent.0)
    .execute(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to create session".to_string(),
        }),
    ))?;

    Ok(Json(AuthResponse {
        token,
        user_id,
        username,
        expires_at: expires_at.to_rfc3339(),
    }))
}
