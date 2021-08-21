#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;

pub use crate::database::connect;

type Result<T> = std::result::Result<T, diesel::result::Error>;
