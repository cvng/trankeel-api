mod testing;

pub mod auth;
pub mod billing;
pub mod companies;
pub mod database;
pub mod documents;
pub mod error;
pub mod files;
pub mod imports;
pub mod leases;
pub mod mailer;
pub mod messages;
pub mod owners;
pub mod payment;
pub mod pdfmaker;
pub mod properties;
pub mod reports;
pub mod tenants;

pub use piteo_data::*;

// TODO: Move ops to lib then remove this.
type Conn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
