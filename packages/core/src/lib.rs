#![deny(clippy::disallowed_method)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

pub mod billing;
pub mod context;
pub mod database;
pub mod dispatcher;
pub mod error;
pub mod handlers;
pub mod mailer;
pub mod messenger;
pub mod pdfmaker;
pub mod providers;
pub mod templates;
