use firebase_admin_auth_rs::jwk_auth::JwkAuth;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use trankeel::config::Config;
use trankeel::AuthId;

const TOKEN_PREFIX: &str = "Bearer";

#[derive(Debug)]
pub enum Error {
    Unauthorized,
}

/// Authorization request guard.
///
/// https://rocket.rs/v0.5-rc/guide/requests/#request-guards
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
        let config = request.rocket().state::<Config>().unwrap();
        let jwk_auth = request.rocket().state::<JwkAuth>().unwrap();

        let maybe_token = request.headers().get_one("authorization");

        let auth_id = match maybe_token {
            Some(token) => {
                // Verify ID token from request in production (release).
                match jwk_auth.verify(&extract_token(token)) {
                    Some(data) => data.claims.sub,
                    _ => return Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
                }
            }
            None => {
                // Try ID token fallback from env in development (debug).
                match config.debug_auth_id.clone() {
                    #[cfg(debug_assertions)]
                    Some(debug_auth_id) => debug_auth_id,
                    _ => return Outcome::Success(Self(None)),
                }
            }
        };

        Outcome::Success(Self(Some(AuthId::new(auth_id))))
    }
}

fn extract_token(token: &str) -> String {
    token[TOKEN_PREFIX.len() + 1..].to_string()
}
