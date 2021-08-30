use crate::database::Conn;
use crate::schema::plan;
use crate::Amount;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Queryable)]
pub struct Plan {
    pub code: String,
    pub price: Option<Amount>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    pub id: uuid::Uuid,
}

// # Queries

pub fn find_by_id(conn: &Conn, id: uuid::Uuid) -> Result<Plan, Error> {
    plan::table.find(id).first(conn).map_err(|err| err.into())
}
