use crate::schema::persons;
use crate::AuthId;
use crate::Conn;
use diesel::dsl::FindBy;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Person;
use piteo_data::PersonId;

// # Queries

pub fn find(conn: &Conn, auth_id: &AuthId) -> Result<Person, Error> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

pub fn person_by_id(conn: &Conn, id: &PersonId) -> Result<Person, Error> {
    persons::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<persons::table, persons::auth_id, &str> {
    persons::table.filter(persons::auth_id.eq(auth_id.inner()))
}
