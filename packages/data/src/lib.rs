#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_newtype;

mod account;
mod address;
mod amount;
mod common;
mod company;
mod datetime;
mod event;
mod file;
mod import;
mod lease;
mod lease_details;
mod legal_identity;
mod lender;
mod payment;
mod payment_notice;
mod person;
mod plan;
mod property;
mod rent;
mod rent_receipt;
mod subscription;
mod summary;
mod task;
mod tenant;

pub mod schema; // Export database schema.

pub use crate::account::*;
pub use crate::address::*;
pub use crate::amount::*;
pub use crate::common::*;
pub use crate::company::*;
pub use crate::datetime::*;
pub use crate::event::*;
pub use crate::file::*;
pub use crate::import::*;
pub use crate::lease::*;
pub use crate::lease_details::*;
pub use crate::legal_identity::*;
pub use crate::lender::*;
pub use crate::payment::*;
pub use crate::payment_notice::*;
pub use crate::person::*;
pub use crate::plan::*;
pub use crate::property::*;
pub use crate::rent::*;
pub use crate::rent_receipt::*;
pub use crate::subscription::*;
pub use crate::summary::*;
pub use crate::tenant::*;
