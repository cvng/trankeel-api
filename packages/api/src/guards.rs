use firebase_admin_auth_rs::jwk_auth::JwkAuth;
use poem::async_trait;
use poem::error::Unauthorized;
use poem::Endpoint;
use poem::IntoResponse;
use poem::Middleware;
use poem::Request;
use poem::Response;
use poem::Result;
use std::sync::Arc;
use trankeel::config::Config;
use trankeel::AuthId;

const TOKEN_HEADER: &str = "Authorization";
const TOKEN_SCHEME: &str = "Bearer";

type Verifier = Arc<JwkAuth>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error verifying ID token")]
    InvalidToken,
}

pub struct AuthIdGuard(Option<AuthId>);

impl AuthIdGuard {
    pub fn new(auth_id: String) -> Self {
        Self(Some(AuthId::new(auth_id)))
    }

    pub fn inner(&self) -> Option<AuthId> {
        self.0.clone()
    }
}

/// A middleware that verify token from HTTP headers.
///
/// https://docs.rs/poem/latest/poem/middleware/trait.Middleware.html
pub struct AuthMiddleware {
    config: Config,
    verifier: Verifier,
}

impl AuthMiddleware {
    pub async fn new(config: Config) -> Self {
        let project_id = config.firebase_project_id.clone().unwrap();

        Self {
            config,
            verifier: Arc::new(JwkAuth::new(project_id).await),
        }
    }
}

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
    type Output = AuthMiddlewareImpl<E>;

    fn transform(&self, endpoint: E) -> Self::Output {
        AuthMiddlewareImpl {
            endpoint,
            config: self.config.clone(),
            verifier: self.verifier.clone(),
        }
    }
}

pub struct AuthMiddlewareImpl<E> {
    endpoint: E,
    config: Config,
    verifier: Verifier,
}

#[async_trait]
impl<E: Endpoint> Endpoint for AuthMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, mut request: Request) -> Result<Self::Output> {
        let config = self.config.clone();
        let verifier = self.verifier.clone();

        let maybe_token = request
            .header(TOKEN_HEADER)
            .map(|token| token[TOKEN_SCHEME.len() + 1..].to_string());

        let auth_id = match maybe_token {
            Some(token) => {
                // Verify ID token from request in production (release).
                match verifier.verify(&token) {
                    Some(data) => data.claims.sub,
                    _ => return Err(Unauthorized(Error::InvalidToken)),
                }
            }
            None => {
                // Try ID token fallback from env in development (debug).
                match config.debug_auth_id {
                    #[cfg(not(feature = "release"))]
                    Some(debug_auth_id) => debug_auth_id,
                    _ => {
                        return self
                            .endpoint
                            .call(request)
                            .await
                            .map(|response| response.into_response())
                    }
                }
            }
        };

        request.set_data(AuthIdGuard::new(auth_id));

        self.endpoint
            .call(request)
            .await
            .map(|response| response.into_response())
    }
}
