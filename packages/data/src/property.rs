use crate::common::Id;
use crate::schema::properties;
use crate::AccountId;
use crate::Address;
use crate::Amount;
use crate::DateTime;
use crate::LenderId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;

// # Types

pub type PropertyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyRoomType {
    Other,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyStatus {
    ForSale,
    Inactive,
    Rented,
    UnderConstruction,
    Unrented,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyBuildPeriodType {
    BeforeY1949,
    FromY1949_Y1974,
    FromY1975_Y1989,
    FromY1990_Y2005,
    FromY2005,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyEnergyClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyGasEmission {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyBuildingLegalStatus {
    Copro,
    Mono,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyHabitationUsageType {
    Habitation,
    Mixte,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyUsageType {
    Collective,
    Individual,
}

#[derive(Clone, Debug, Insertable, Queryable)]
#[table_name = "properties"]
pub struct Property {
    pub id: PropertyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub address: Address,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub name: String,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub tax: Option<Amount>,
    pub room_count: PropertyRoomType,
    pub status: Option<PropertyStatus>,
    pub surface: f32,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
    pub lender_id: LenderId,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "properties"]
pub struct PropertyData {
    pub id: PropertyId,
    pub account_id: Option<AccountId>,
    pub address: Option<Address>,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub tax: Option<Amount>,
    pub room_count: Option<PropertyRoomType>,
    pub status: Option<PropertyStatus>,
    pub surface: Option<f32>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
    pub lender_id: Option<LenderId>,
}
