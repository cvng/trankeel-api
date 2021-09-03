use crate::database::Conn;
use crate::schema::plan;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Plan;

// # Queries

pub fn find_by_id(conn: &Conn, id: Id) -> Result<Plan, Error> {
    plan::table.find(id).first(conn).map_err(|err| err.into())
}
