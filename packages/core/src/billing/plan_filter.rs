use crate::schema::plan;
use crate::Conn;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Plan;
use piteo_data::PlanId;

// # Queries

pub fn find_by_id(conn: &Conn, id: PlanId) -> Result<Plan, Error> {
    plan::table.find(id).first(conn).map_err(|err| err.into())
}