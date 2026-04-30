use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use super::{generate_token, hash_token, get_email_token_expiry, send_email, hash_password, ErrorResponse};

#[derive(Deserialize)]
pub struct RequestPasswordResetRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct RequestPasswordResetResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

/// Sends a password reset email to the user
#[post("/auth/password-reset-request", format = "json", data = "<req>")]
pub async fn request_password_reset(
    pool: &State<Pool<MySql>>,
    req: Json<RequestPasswordResetRequest>,
) -> Result<Json<RequestPasswordResetResponse>, (Status, Json<ErrorResponse>)> {
    // Check if user exists
    let user: Option<(i32,)> = sqlx::query_as(
        "SELECT user_id FROM users WHERE email = ?"
    )
    .bind(&req.email)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Database error".to_string(),
        }),
    ))?;

    let (user_id,) = user.ok_or((
        Status::NotFound,
        Json(ErrorResponse {
            error: "Email not found".to_string(),
        }),
    ))?;

    // Generate reset token
    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at = get_email_token_expiry();

    // Delete any existing password reset tokens for this user
    sqlx::query(
        "DELETE FROM user_tokens WHERE user_id = ? AND token_type = 'password_reset'"
    )
    .bind(user_id)
    .execute(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Database error".to_string(),
        }),
    ))?;

    // Insert new reset token
    sqlx::query(
        "INSERT INTO user_tokens (token_hash, user_id, token_type, expires_at) VALUES (?, ?, 'password_reset', ?)"
    )
    .bind(&token_hash)
    .bind(user_id)
    .bind(expires_at.to_rfc3339())
    .execute(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to create reset token".to_string(),
        }),
    ))?;

    // Send email with reset link
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let reset_link = format!("{}/auth/reset-password?email={}&token={}", frontend_url, urlencoding::encode(&req.email), token);
    
    let subject = "Reset Your Password - TS Lyric Completion";
    let body = format!(
        "Click the link below to reset your password:\n\n{}\n\nThis link expires in 24 hours.\n\nIf you didn't request this, please ignore this email.",
        reset_link
    );

    send_email(&req.email, subject, &body).map_err(|e| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: format!("Failed to send email: {}", e),
        }),
    ))?;

    Ok(Json(RequestPasswordResetResponse {
        message: "Password reset email sent".to_string(),
    }))
}

/// Resets the user's password with a valid token
#[post("/auth/reset-password", format = "json", data = "<req>")]
pub async fn reset_password(
    pool: &State<Pool<MySql>>,
    req: Json<ResetPasswordRequest>,
) -> Result<Json<ResetPasswordResponse>, (Status, Json<ErrorResponse>)> {
    // Validate password
    if req.new_password.len() < 8 {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Password must be at least 8 characters".to_string(),
            }),
        ));
    }

    // Check if user exists with this email
    let user: Option<(i32,)> = sqlx::query_as(
        "SELECT user_id FROM users WHERE email = ?"
    )
    .bind(&req.email)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Database error".to_string(),
        }),
    ))?;

    let (user_id,) = user.ok_or((
        Status::NotFound,
        Json(ErrorResponse {
            error: "Email not found".to_string(),
        }),
    ))?;

    // Hash the provided token to compare with database
    let token_hash = super::hash_token(&req.token);

    // Check if token exists, matches user_id, is not expired, and is a password_reset token
    let token: Option<(String,)> = sqlx::query_as(
        "SELECT token_hash FROM user_tokens WHERE token_hash = ? AND user_id = ? AND token_type = 'password_reset' AND expires_at > NOW()"
    )
    .bind(&token_hash)
    .bind(user_id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Database error".to_string(),
        }),
    ))?;

    if token.is_none() {
        return Err((
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "Invalid or expired reset token".to_string(),
            }),
        ));
    }

    // Hash the new password
    let password_hash = hash_password(&req.new_password).map_err(|_| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: "Failed to hash password".to_string(),
        }),
    ))?;

    // Update user's password
    sqlx::query("UPDATE users SET password_hash = ? WHERE user_id = ?")
        .bind(&password_hash)
        .bind(user_id)
        .execute(pool.inner())
        .await
        .map_err(|_| (
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Failed to reset password".to_string(),
            }),
        ))?;

    // Delete the used token
    sqlx::query("DELETE FROM user_tokens WHERE token_hash = ?")
        .bind(&token_hash)
        .execute(pool.inner())
        .await
        .map_err(|_| (
            Status::InternalServerError,
            Json(ErrorResponse {
                error: "Failed to cleanup token".to_string(),
            }),
        ))?;

    // Send confirmation email
    send_email(&req.email, "Password changed", "Your password to log in to tslyriccompletion.com has been changed successfully.").map_err(|e| (
        Status::InternalServerError,
        Json(ErrorResponse {
            error: format!("Failed to send email: {}", e),
        }),
    ))?;

    Ok(Json(ResetPasswordResponse {
        message: "Password reset successfully".to_string(),
    }))
}
