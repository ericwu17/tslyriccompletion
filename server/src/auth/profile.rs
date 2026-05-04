use super::bearer_token::BearerToken;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::{MySql, Pool};

use super::ErrorResponse;

#[derive(Serialize)]
pub struct UserProfile {
    pub username: String,
    pub email: Option<String>,
    pub email_verified: bool,
}

/// Get the current user's profile
#[get("/auth/profile")]
pub async fn get_profile(
    pool: &State<Pool<MySql>>,
    bearer_token: BearerToken,
) -> Result<Json<UserProfile>, (Status, Json<ErrorResponse>)> {
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

    // Get the user's profile information
    let user: Option<(String, Option<String>, bool)> =
        sqlx::query_as("SELECT username, email, email_verified FROM users WHERE user_id = ?")
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

    let (username, email, email_verified) = user.ok_or((
        Status::NotFound,
        Json(ErrorResponse {
            error: "User not found".to_string(),
        }),
    ))?;

    Ok(Json(UserProfile {
        username,
        email,
        email_verified,
    }))
}
