use crate::error::Context;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use eyre::Error;

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
