use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

/// Database pool.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Build connection pool.
pub fn build_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect(&format!("Error connecting to {}", database_url))
}
