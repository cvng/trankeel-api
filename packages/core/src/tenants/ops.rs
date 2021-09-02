use crate::auth;
use crate::database::Conn;
use crate::schema::tenant;
use crate::AuthId;
use crate::DateTime;
use crate::Tenant;
use diesel::insert_into;
use diesel::prelude::*;
use eyre::Error;
use serde::Deserialize;

// # Inputs

#[derive(Deserialize, Insertable)]
#[table_name = "tenant"]
pub struct TenantInput {
    pub apl: Option<bool>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub visale_id: Option<String>,
}

// # Operations

pub fn create_tenant(conn: &Conn, auth_id: &AuthId, input: TenantInput) -> Result<Tenant, Error> {
    let account = auth::get_account_by_auth_id(conn, auth_id)?;

    let new_tenant = insert_into(tenant::table)
        .values((
            tenant::account_id.eq(account.id),
            tenant::birthdate.eq(input.birthdate),
            tenant::email.eq(input.email),
            tenant::first_name.eq(input.first_name),
            tenant::last_name.eq(input.last_name),
        ))
        .get_result(conn)?;

    Ok(new_tenant)
}
