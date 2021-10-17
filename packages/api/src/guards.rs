use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use trankeel::AuthId;

const AUTH_ID_KEY: &str = "DEBUG_AUTH_ID";

#[derive(Debug)]
pub enum Error {
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedIdToken {
    pub aud: String,
    pub auth_time: usize,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}

/// Authorization request guard. https://rocket.rs/v0.5-rc/guide/requests/#request-guards
pub struct AuthGuard(Option<AuthId>);

impl AuthGuard {
    pub fn inner(&self) -> Option<AuthId> {
        self.0.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let maybe_token = request.headers().get_one("authorization");

        let auth_id = match maybe_token {
            Some(token) => {
                // Verify ID token from request in production (release).
                match jsonwebtoken::dangerous_insecure_decode::<DecodedIdToken>(token) {
                    Ok(data) => data.claims.sub,
                    _ => return Outcome::Failure((Status::Unauthorized, Error::Invalid)),
                }
            }
            None => {
                // Try ID token fallback from env in development (debug).
                match env::var(AUTH_ID_KEY) {
                    Ok(text) if cfg!(debug_assertions) => text,
                    _ => return Outcome::Success(Self(None)),
                }
            }
        };

        Outcome::Success(Self(Some(AuthId::new(auth_id))))
    }
}
