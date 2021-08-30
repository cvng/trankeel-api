use crate::database::Conn;
use crate::schema::tenant;
use crate::schema::user;
use crate::AuthId;
use crate::DateTime;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;

pub enum TenantStatus {
    Gone,
    Late,
    New,
    Uptodate,
}

// # Models

#[derive(Clone, Queryable)]
pub struct Tenant {
    pub account_id: Id,
    pub apl: bool,
    pub auth_id: Option<crate::AuthId>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub id: Id,
    pub lease_id: Option<Id>,
    pub visale_id: Option<String>,
}

// # Queries

pub fn all_tenants(conn: &Conn, auth_id: &AuthId, id: Option<Id>) -> Result<Vec<Tenant>, Error> {
    let auth_id = auth_id.clone();

    let query = tenant::table
        .select(tenant::all_columns)
        .left_join(user::table.on(user::accountId.eq(tenant::accountId.nullable())))
        .filter(user::authId.eq(auth_id.inner()));

    if let Some(id) = id {
        return query
            .filter(tenant::id.eq(id))
            .load(conn)
            .map_err(|err| err.into());
    }

    query.load(conn).map_err(|err| err.into())
}
