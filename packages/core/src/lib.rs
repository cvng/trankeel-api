#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;

pub use crate::database::establish_connection;

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub struct Context {
    pub conn: diesel::PgConnection,
}
