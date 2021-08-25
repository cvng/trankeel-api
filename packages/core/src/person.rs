use crate::database::Conn;
use crate::schema::user;
use crate::Address;
use crate::AuthId;
use diesel::dsl::FindBy;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Queryable)]
pub struct Person {
    pub auth_id: AuthId,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<Address>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub id: uuid::Uuid,
    pub phone_number: Option<String>,
    pub account_id: Option<uuid::Uuid>,
}

// # Queries

pub fn first_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Person, Error> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<user::table, user::authId, &str> {
    user::table.filter(user::authId.eq(auth_id.inner()))
}
