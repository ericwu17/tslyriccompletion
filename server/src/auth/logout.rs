use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use sqlx::{MySql, Pool};

use super::{hash_token, ErrorResponse};

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

#[post("/auth/logout", format = "json", data = "<req>")]
pub async fn logout(
    pool: &State<Pool<MySql>>,
    req: Json<LogoutRequest>,
) -> Result<(), (Status, Json<ErrorResponse>)> {
    let token_hash = hash_token(&req.token);

    // Check if token exists
    let session: Option<(String,)> =
        sqlx::query_as("SELECT token_hash FROM user_sessions WHERE token_hash = ?")
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

    if session.is_none() {
        return Err((
            Status::Unauthorized,
            Json(ErrorResponse {
                error: "Invalid token".to_string(),
            }),
        ));
    }

    // Delete the session
    sqlx::query("DELETE FROM user_sessions WHERE token_hash = ?")
        .bind(&token_hash)
        .execute(pool.inner())
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: "Failed to logout".to_string(),
                }),
            )
        })?;

    Ok(())
}
