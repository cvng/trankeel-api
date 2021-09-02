use crate::database::Conn;
use crate::schema::account;
use crate::schema::user;
use crate::AuthId;
use crate::DateTime;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Queryable)]
pub struct Account {
    pub plan_id: Option<Id>,
    pub status: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub trial_end: Option<DateTime>,
    pub owner_id: String,
    pub id: Id,
}

// # Queries

pub fn find_by_id(conn: &Conn, id: Id) -> Result<Account, Error> {
    account::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}

pub fn get_account_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Account, Error> {
    account::table
        .select(account::all_columns)
        .left_join(user::table.on(user::accountId.eq(account::id.nullable())))
        .filter(user::authId.eq(&auth_id.inner()))
        .first(conn)
        .map_err(|err| err.into())
}
