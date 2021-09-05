use crate::schema::account;
use crate::schema::user;
use crate::AuthId;
use crate::Conn;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountId;

// # Queries

pub fn find_by_id(conn: &Conn, id: AccountId) -> Result<Account, Error> {
    account::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}

pub fn get_account_by_auth_id(conn: &Conn, auth_id: AuthId) -> Result<Account, Error> {
    account::table
        .select(account::all_columns)
        .left_join(user::table.on(user::account_id.eq(account::id.nullable())))
        .filter(user::auth_id.eq(&auth_id.inner()))
        .first(conn)
        .map_err(|err| err.into())
}
