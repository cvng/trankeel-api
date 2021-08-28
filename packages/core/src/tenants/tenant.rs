use crate::database::Conn;
use crate::schema::tenant;
use crate::schema::user;
use crate::AuthId;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Clone, Queryable)]
pub struct Tenant {
    pub account_id: uuid::Uuid,
    pub apl: bool,
    pub auth_id: Option<crate::AuthId>,
    pub birthdate: chrono::NaiveDateTime,
    pub birthplace: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub id: uuid::Uuid,
    pub lease_id: Option<uuid::Uuid>,
    pub visale_id: Option<String>,
}

// # Queries

pub fn load_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Tenant>, Error> {
    tenant::table
        .select(tenant::all_columns)
        .left_join(user::table.on(user::accountId.eq(tenant::accountId.nullable())))
        .filter(user::authId.eq(&auth_id.inner()))
        .load(conn)
        .map_err(|err| err.into())
}
