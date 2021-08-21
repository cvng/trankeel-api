#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;

pub type Error = diesel::result::Error;

pub type Result<T> = std::result::Result<T, Error>;
