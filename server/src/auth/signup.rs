use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use super::{
    generate_token, get_session_expiry, hash_password, hash_token, is_valid_username_format,
    ErrorResponse, UserAgent,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
    pub expires_at: String,
}

#[post("/auth/signup", format = "json", data = "<req>")]
pub async fn signup(
    pool: &State<Pool<MySql>>,
    user_agent: UserAgent,
    req: Json<SignupRequest>,
) -> Result<Json<SignupResponse>, (Status, Json<ErrorResponse>)> {
    // Validation
    if req.username.len() < 6 || req.username.len() > 50 {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Username must be between 6 and 50 characters".to_string(),
            }),
        ));
    }

    if !is_valid_username_format(&req.username) {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Username can only contain letters, numbers, underscores, and hyphens"
                    .to_string(),
            }),
        ));
    }

    if req.password.len() < 8 {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Password must be at least 8 characters".to_string(),
            }),
        ));
    }

    // Validate email format (basic check)
    if !req.email.contains('@') || req.email.len() > 100 {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Invalid email address".to_string(),
            }),
        ));
    }

    // Check if username already exists
    let existing_user: Option<(i32,)> =
        sqlx::query_as("SELECT user_id FROM users WHERE username = ?")
            .bind(&req.username)
            .fetch_optional(pool.inner())
            .await
            .map_err(|_| {
                (
                    Status::InternalServerError,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    if existing_user.is_some() {
        return Err((
            Status::Conflict,
            Json(ErrorResponse {
                error: "Username already exists".to_string(),
            }),
        ));
    }

    // Check if email already exists
    let existing_email: Option<(i32,)> =
        sqlx::query_as("SELECT user_id FROM users WHERE email = ?")
            .bind(&req.email)
            .fetch_optional(pool.inner())
            .await
            .map_err(|_| {
                (
                    Status::InternalServerError,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    if existing_email.is_some() {
        return Err((
            Status::Conflict,
            Json(ErrorResponse {
                error: "Email already registered".to_string(),
            }),
        ));
    }

    // Check that username is not someone else's email
    let username_as_email: Option<(i32,)> =
        sqlx::query_as("SELECT user_id FROM users WHERE email = ?")
            .bind(&req.username)
            .fetch_optional(pool.inner())
            .await
            .map_err(|_| {
                (
                    Status::InternalServerError,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    if username_as_email.is_some() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Username cannot be someone else's email".to_string(),
            }),
        ));
    }

    // Check that email is not someone else's username
    let email_as_username: Option<(i32,)> =
        sqlx::query_as("SELECT user_id FROM users WHERE username = ?")
            .bind(&req.email)
            .fetch_optional(pool.inner())
            .await
            .map_err(|_| {
                (
                    Status::InternalServerError,
                    Json(ErrorResponse {
                        error: "Database error".to_string(),
                    }),
                )
            })?;

    if email_as_username.is_some() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Email cannot be someone else's username".to_string(),
            }),
        ));
    }

    // Hash password
    let password_hash = hash_password(&req.password).map_err(|_| {
        (
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            }),
        )
    })?;

    // Insert user into database
    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, NOW(), NOW())"
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .execute(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to create user".to_string(),
        }),
    ))?;

    let user_id = result.last_insert_id() as i32;

    // Generate session token
    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at = get_session_expiry();

    // Insert session into user_sessions table
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

    Ok(Json(SignupResponse {
        token,
        user_id,
        username: req.username.clone(),
        expires_at: expires_at.to_rfc3339(),
    }))
}
