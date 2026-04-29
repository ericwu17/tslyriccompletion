pub mod login;
pub mod logout;
pub mod signup;

use rand::Rng;
use sha1::{Digest, Sha1};
use chrono::Duration;
use chrono::Utc;
use rocket::request::{FromRequest, Request, Outcome};

/// Request guard to extract User-Agent header
pub struct UserAgent(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_agent = request
            .headers()
            .get_one("User-Agent")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        Outcome::Success(UserAgent(user_agent))
    }
}

/// Hashes a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verifies a password against a bcrypt hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

/// Generates a cryptographically secure random token
pub fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..16);
            format!("{:x}", idx)
        })
        .collect();
    token
}

/// Hashes a token using SHA1 for storage in database
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Returns the expiration datetime for a new session (30 days from now)
pub fn get_session_expiry() -> chrono::DateTime<Utc> {
    Utc::now() + Duration::days(30)
}

/// Validates that a username contains only allowed characters [a-zA-Z0-9_-]
pub fn is_valid_username_format(username: &str) -> bool {
    username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') && username.len() >= 6
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
    pub expires_at: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}
