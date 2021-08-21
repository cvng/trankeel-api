#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;
pub mod properties;

pub type Error = diesel::result::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Authentication ID
pub struct AuthId(String);

/// Try reading ID token from env in debug.
pub fn auth_id_fallback_from_env() -> AuthId {
    dotenv::dotenv().ok();

    AuthId(std::env::var("FIREBASE_ADMIN_USER_ID").expect("FIREBASE_ADMIN_USER_ID must be set"))
}
