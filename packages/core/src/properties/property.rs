use crate::database::Conn;
use crate::schema::property;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Property;

// # Queries

pub fn all_properties(
    conn: &Conn,
    auth_id: &AuthId,
    id: Option<Id>,
) -> Result<Vec<Property>, Error> {
    let auth_id = auth_id.clone();

    let query = property::table
        .select(property::all_columns)
        .left_join(user::table.on(user::accountId.eq(property::accountId)))
        .filter(user::authId.eq(auth_id.inner()));

    if let Some(id) = id {
        return query
            .filter(property::id.eq(id))
            .load(conn)
            .map_err(|err| err.into());
    }

    query.load(conn).map_err(|err| err.into())
}
