use crate::database::Conn;
use crate::schema::property;
use crate::schema::user;
use crate::Address;
use crate::AuthId;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Clone, Queryable)]
pub struct Property {
    pub account_id: Option<uuid::Uuid>,
    pub address: Address,
    pub build_period: Option<String>,
    pub building_legal_status: Option<String>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<String>,
    pub equipments: Option<String>,
    pub gas_emission: Option<String>,
    pub heating_method: Option<String>,
    pub housing_type: Option<String>,
    pub name: String,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub tax: Option<f64>,
    pub room_count: String,
    pub status: Option<String>,
    pub surface: i32,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<String>,
    pub water_heating_method: Option<String>,
    pub id: uuid::Uuid,
    pub lender_id: uuid::Uuid,
}

// # Queries

pub fn load_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Property>, Error> {
    property::table
        .select(property::all_columns)
        .left_join(user::table.on(user::accountId.eq(property::accountId)))
        .filter(user::authId.eq(&auth_id.inner()))
        .load(conn)
        .map_err(|err| err.into())
}
