mod database;
mod payment;

use crate::database::Database;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use payment::Stripe;
use piteo_core::auth;
use piteo_core::auth::ops::UserWithAccountInput;
use piteo_core::error::Context;
use piteo_core::error::Error;
use piteo_core::tenants;
use piteo_core::tenants::ops::TenantInput;
use piteo_core::AuthId;
use piteo_core::Person;
use piteo_core::Tenant;
use piteo_core::TenantId;

/// Database pool.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Database connection.
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

/// Build connection pool.
pub fn build_connection_pool(database_url: &str) -> Result<DbPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .context(format!("Error connecting to {}", database_url))
}

pub fn all_tenants(
    conn: &Conn,
    auth_id: AuthId,
    id: Option<TenantId>,
) -> Result<Vec<Tenant>, Error> {
    tenants::all_tenants(Database::new(conn), auth_id, id)
}

pub fn create_tenant(conn: &Conn, auth_id: AuthId, input: TenantInput) -> Result<Tenant, Error> {
    tenants::ops::create_tenant(Database::new(conn), auth_id, input)
}

pub fn create_user_with_account(conn: &Conn, input: UserWithAccountInput) -> Result<Person, Error> {
    auth::ops::create_user_with_account(Database::new(conn), Stripe, input)
}

fn wip() -> Error {
    Error::msg("wip!()")
}
