#[macro_use]
extern crate diesel;

mod common;
mod database;
mod locale;
mod schema;

pub mod auth;
pub mod billing;
pub mod error;
pub mod files;
pub mod leases;
pub mod properties;
pub mod reports;
pub mod tenants;

pub use crate::auth::Account;
pub use crate::auth::AuthId;
pub use crate::auth::Person;
pub use crate::billing::Plan;
pub use crate::common::Address;
pub use crate::common::Amount;
pub use crate::common::DateTime;
pub use crate::common::Id;
pub use crate::database::build_connection_pool;
pub use crate::database::DbPool;
pub use crate::error::Error;
pub use crate::leases::Lease;
pub use crate::leases::LeaseData;
pub use crate::leases::LeaseStatus;
pub use crate::leases::LeaseType;
pub use crate::leases::Rent;
pub use crate::leases::RentStatus;
pub use crate::properties::Lender;
pub use crate::properties::Property;
pub use crate::reports::Summary;
pub use crate::tenants::Tenant;
pub use crate::tenants::TenantStatus;
