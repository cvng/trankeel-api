use crate::database::Conn;
use crate::schema::lease;
use crate::schema::user;
use crate::AuthId;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Lease;

// # Queries

pub fn all_leases(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Lease>, Error> {
    lease::table
        .select(lease::all_columns)
        .left_join(user::table.on(user::accountId.eq(lease::accountId.nullable())))
        .filter(user::authId.eq(&auth_id.inner()))
        .load(conn)
        .map_err(|err| err.into())
}
