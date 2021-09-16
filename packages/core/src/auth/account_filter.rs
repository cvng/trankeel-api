use crate::schema::accounts;
use crate::schema::persons;
use crate::AuthId;
use crate::Conn;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountId;

// # Queries

pub fn find_by_id(conn: &Conn, id: AccountId) -> Result<Account, Error> {
    accounts::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}

pub fn get_account_by_auth_id(conn: &Conn, auth_id: AuthId) -> Result<Account, Error> {
    accounts::table
        .select(accounts::all_columns)
        .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
        .filter(persons::auth_id.eq(&auth_id.inner()))
        .first(conn)
        .map_err(|err| err.into())
}
