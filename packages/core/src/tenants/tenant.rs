use crate::database::Conn;
use crate::schema::tenant;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Tenant;

// # Queries

pub fn all_tenants(conn: &Conn, auth_id: &AuthId, id: Option<Id>) -> Result<Vec<Tenant>, Error> {
    let auth_id = auth_id.clone();

    let query = tenant::table
        .select(tenant::all_columns)
        .left_join(user::table.on(user::accountId.eq(tenant::account_id.nullable())))
        .filter(user::authId.eq(auth_id.inner()));

    if let Some(id) = id {
        return query
            .filter(tenant::id.eq(id))
            .load(conn)
            .map_err(|err| err.into());
    }

    query.load(conn).map_err(|err| err.into())
}
