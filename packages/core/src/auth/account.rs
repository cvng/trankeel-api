use crate::database::Conn;
use crate::schema::account;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Queryable)]
pub struct Account {
    pub plan_id: Option<uuid::Uuid>,
    pub status: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub trial_end: Option<chrono::NaiveDateTime>,
    pub owner_id: String,
    pub id: uuid::Uuid,
}

// # Queries

pub fn find_by_id(conn: &Conn, id: uuid::Uuid) -> Result<Account, Error> {
    account::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}
