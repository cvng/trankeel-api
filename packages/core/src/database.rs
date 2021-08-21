use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn connect() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
