use crate::schema::leases;
use crate::schema::persons;
use crate::AuthId;
use crate::Conn;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Lease;

// # Queries

pub fn all_leases(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Lease>, Error> {
    leases::table
        .select(leases::all_columns)
        .left_join(persons::table.on(persons::account_id.eq(leases::account_id)))
        .filter(persons::auth_id.eq(&auth_id.inner()))
        .load(conn)
        .map_err(|err| err.into())
}
