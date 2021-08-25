use piteo_core::AuthId;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use std::env;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
}

/// Token request guard. https://rocket.rs/v0.5-rc/guide/requests/#request-guards
pub struct Token(pub AuthId);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Error;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("authorization") {
            Some(auth_id) => Outcome::Success(Self(AuthId::new(auth_id.into()))),
            None => {
                // Fallback to authentication ID from env in debug.
                if cfg!(debug_assertions) {
                    if let Ok(auth_id) = env::var("FIREBASE_ADMIN_USER_ID") {
                        return Outcome::Success(Self(AuthId::new(auth_id)));
                    }
                }
                Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
            }
        }
    }
}
