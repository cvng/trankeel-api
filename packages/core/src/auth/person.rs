use crate::database::Conn;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use diesel::dsl::FindBy;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Person;

// # Queries

pub fn find(conn: &Conn, auth_id: &AuthId) -> Result<Person, Error> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

pub fn person_by_id(conn: &Conn, id: &Id) -> Result<Person, Error> {
    user::table.find(id).first(conn).map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<user::table, user::authId, &str> {
    user::table.filter(user::authId.eq(auth_id.inner()))
}
