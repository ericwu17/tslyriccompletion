use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::auth::bearer_token::BearerToken;

use super::{generate_token, get_email_token_expiry, hash_token, send_email, ErrorResponse};

#[derive(Deserialize)]
pub struct RequestEmailVerificationRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct RequestEmailVerificationResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
}

/// Sends an email verification email to the user
#[post("/auth/verify-email-request")]
pub async fn request_email_verification(
    pool: &State<Pool<MySql>>,
    bearer_token: BearerToken,
) -> Result<Json<RequestEmailVerificationResponse>, (Status, Json<ErrorResponse>)> {
    let token_hash = super::hash_token(&bearer_token.0);

    // Look up the session to get the user_id
    let session: Option<(i32,)> = sqlx::query_as(
        "SELECT user_id FROM user_sessions WHERE token_hash = ? AND expires_at > NOW()",
    )
    .bind(&token_hash)
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

    let (user_id,) = session.ok_or((
        Status::Unauthorized,
        Json(ErrorResponse {
            error: "Invalid or expired session token".to_string(),
        }),
    ))?;

    // Check user's email and email verification status
    let db_email_res: Option<(String, bool)> = sqlx::query_as(
        "SELECT email, email_verified FROM users WHERE user_id = ? AND email IS NOT NULL",
    )
    .bind(user_id)
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

    let (email, email_verified) = db_email_res.ok_or((
        Status::NotFound,
        Json(ErrorResponse {
            error: "Email not found".to_string(),
        }),
    ))?;

    // If already verified, return error
    if email_verified {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Email is already verified".to_string(),
            }),
        ));
    }

    // Generate verification token
    let email_verify_token = generate_token();
    let email_verify_token_hash = hash_token(&email_verify_token);
    let expires_at = get_email_token_expiry();

    // Delete any existing verification tokens for this user
    sqlx::query("DELETE FROM user_tokens WHERE user_id = ? AND token_type = 'email_verification'")
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
        "Click the link below to verify your email:\n\n{}\n\n This link expires in 24 hours.",
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

    Ok(Json(RequestEmailVerificationResponse {
        message: "Verification email sent".to_string(),
    }))
}

/// Verifies the user's email with a token
#[post("/auth/verify-email", format = "json", data = "<req>")]
pub async fn verify_email(
    pool: &State<Pool<MySql>>,
    req: Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, (Status, Json<ErrorResponse>)> {
    // Check if user exists with this email
    let user: Option<(i32,)> =
        sqlx::query_as("SELECT user_id FROM users WHERE email = ? AND email IS NOT NULL")
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

    let (user_id,) = user.ok_or((
        Status::NotFound,
        Json(ErrorResponse {
            error: "Email not found".to_string(),
        }),
    ))?;

    // Hash the provided token to compare with database
    let token_hash = super::hash_token(&req.token);

    // Check if token exists, matches user_id, is not expired, and is an email_verification token
    let token: Option<(String,)> = sqlx::query_as(
        "SELECT token_hash FROM user_tokens WHERE token_hash = ? AND user_id = ? AND token_type = 'email_verification' AND expires_at > NOW()"
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
                error: "Invalid or expired verification token".to_string(),
            }),
        ));
    }

    // Update user's email_verified field
    sqlx::query("UPDATE users SET email_verified = 1 WHERE user_id = ?")
        .bind(user_id)
        .execute(pool.inner())
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Failed to verify email".to_string(),
                }),
            )
        })?;

    // Delete the used token
    sqlx::query("DELETE FROM user_tokens WHERE token_hash = ?")
        .bind(&token_hash)
        .execute(pool.inner())
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Failed to cleanup token".to_string(),
                }),
            )
        })?;

    // Send confirmation email to user
    send_email(
        &req.email,
        "Email Verification Success",
        "Your email to log in to tslyriccompletion.com has been verified successfully.",
    )
    .map_err(|e| {
        (
            Status::InternalServerError,
            Json(ErrorResponse {
                error: format!("Failed to send email: {}", e),
            }),
        )
    })?;

    Ok(Json(VerifyEmailResponse {
        message: "Email verified successfully".to_string(),
    }))
}
