use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

/// Authorization header extractor for Bearer tokens
pub struct BearerToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    "Missing Authorization header".to_string(),
                ))
            }
        };

        if !auth_header.starts_with("Bearer ") {
            return Outcome::Failure((
                Status::Unauthorized,
                "Invalid Authorization header format".to_string(),
            ));
        }

        let token = auth_header
            .strip_prefix("Bearer ")
            .unwrap_or("")
            .to_string();
        Outcome::Success(BearerToken(token))
    }
}
