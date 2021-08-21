use crate::schema::property;
use crate::schema::user;
use crate::Context;
use crate::Result;
use diesel::prelude::*;

// # Models

#[derive(Clone, Queryable)]
pub struct Property {
    pub account_id: Option<uuid::Uuid>,
    // pub address: Jsonb,
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

pub fn load_by_auth_id(ctx: &Context) -> Result<Vec<Property>> {
    property::table
        .select(property::all_columns)
        .left_join(user::table.on(user::accountId.eq(property::accountId)))
        .filter(user::authId.eq(&ctx.auth_id.0))
        .load(&ctx.db_pool.get().unwrap())
}
