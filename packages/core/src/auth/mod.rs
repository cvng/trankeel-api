use crate::database::Conn;
use crate::schema::user;
use crate::Result;
use diesel::dsl::FindBy;
use diesel::prelude::*;

// # Models

/// Authentication ID
pub struct AuthId(pub(crate) String);

impl AuthId {
    pub fn new(auth_id: &str) -> Self {
        Self(auth_id.into())
    }
}

#[derive(Queryable)]
pub struct Person {
    pub auth_id: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    // pub address: Option<Address>, // TODO: https://github.com/diesel-rs/diesel/issues/1950
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub id: uuid::Uuid,
    pub phone_number: Option<String>,
    pub account_id: Option<uuid::Uuid>,
}

// # Queries

pub fn first_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Person> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<user::table, user::authId, &str> {
    user::table.filter(user::authId.eq(auth_id.0.as_str()))
}
