use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::auth::{get_email_token_expiry, send_email};

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
    // TODO: emails are optional
    let maybe_email = if req.email.is_empty() {
        None
    } else {
        Some(req.email.clone())
    };

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
    if let Some(email) = maybe_email.clone() {
        if !email.contains('@') || email.len() > 100 {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse {
                    error: "Invalid email address".to_string(),
                }),
            ));
        }

        // Check if email already exists
        let existing_email: Option<(i32,)> =
            sqlx::query_as("SELECT user_id FROM users WHERE email = ?")
                .bind(&email)
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

        // Check that email is not someone else's username
        let email_as_username: Option<(i32,)> =
            sqlx::query_as("SELECT user_id FROM users WHERE username = ?")
                .bind(&email)
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
    .bind(&maybe_email)
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

    if let Some(email) = maybe_email.clone() {
        // Generate verification token
        let email_verify_token = generate_token();
        let email_verify_token_hash = hash_token(&email_verify_token);
        let expires_at = get_email_token_expiry();

        // Delete any existing verification tokens for this user
        sqlx::query(
            "DELETE FROM user_tokens WHERE user_id = ? AND token_type = 'email_verification'",
        )
        .bind(user_id)
        .execute(pool.inner())
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
        })?;

        // Insert new verification token
        sqlx::query(
        "INSERT INTO user_tokens (token_hash, user_id, token_type, expires_at) VALUES (?, ?, 'email_verification', ?)"
    )
    .bind(&email_verify_token_hash)
    .bind(user_id)
    .bind(expires_at.to_rfc3339())
    .execute(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to create verification token".to_string(),
        }),
    ))?;

        // Send email with verification link
        let frontend_url =
            std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let verification_link = format!(
            "{}/auth/verify-email?email={}&token={}",
            frontend_url,
            urlencoding::encode(&email),
            email_verify_token
        );

        let subject = "Verify Your Email";
        let body = format!(
        "Thanks for signing up for a Taylor Swift Lyric Completion Account! Your username is: {}\n\nClick the link below to verify your email:\n\n{}\n\n. This link expires in 24 hours.",
        req.username,
        verification_link,
    );

        send_email(&email, subject, &body).map_err(|e| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: format!("Failed to send email: {}", e),
                }),
            )
        })?;
    }

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
