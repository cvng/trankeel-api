pub mod create_user_with_account;
pub mod ops;

use crate::schema::account;
use crate::schema::user;
use crate::AuthId;
use crate::Conn;
use crate::Id;
use diesel::dsl::FindBy;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Account;
use piteo_data::Person;

// # Queries

pub fn find_by_id(conn: &Conn, id: Id) -> Result<Account, Error> {
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

// # Queries

pub fn find(conn: &Conn, auth_id: &AuthId) -> Result<Person, Error> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

pub fn person_by_id(conn: &Conn, id: &Id) -> Result<Person, Error> {
    user::table.find(id).first(conn).map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<user::table, user::auth_id, &str> {
    user::table.filter(user::auth_id.eq(auth_id.inner()))
}
