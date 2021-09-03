use crate::database::Conn;
use crate::schema::company;
use crate::PersonId;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Company;

// # Queries

pub fn find(conn: &Conn, id: &PersonId) -> Result<Company, Error> {
    company::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}
