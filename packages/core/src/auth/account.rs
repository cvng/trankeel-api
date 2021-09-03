use crate::database::Conn;
use crate::schema::account;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Account;

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
