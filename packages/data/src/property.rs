use crate::common::Id;
use crate::schema::property;
use crate::AccountId;
use crate::Address;
use crate::Amount;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DieselEnum, Enum)]
pub enum PropertyBuildPeriodType {
    BeforeY1949,
    FromY1949Y1974,
    FromY1975Y1989,
    FromY1990Y2005,
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

#[derive(Clone, Insertable, Queryable)]
#[table_name = "property"]
pub struct Property {
    pub account_id: Option<AccountId>,
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
    pub surface: f64,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
    pub id: PropertyId,
    pub lender_id: LenderId,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "property"]
pub struct PropertyData {
    pub id: PropertyId,
    pub account_id: Option<Id>,
    pub address: Option<Address>,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub lender_id: Option<LenderId>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub room_count: Option<PropertyRoomType>,
    pub status: Option<PropertyStatus>,
    pub surface: Option<f64>,
    pub tax: Option<Amount>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
}

// # Impls

impl From<String> for PropertyRoomType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "OTHER" => Self::Other,
            "T1" => Self::T1,
            "T2" => Self::T2,
            "T3" => Self::T3,
            "T4" => Self::T4,
            "T5" => Self::T5,
            "T6" => Self::T6,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "FOR_SALE" => Self::ForSale,
            "INACTIVE" => Self::Inactive,
            "RENTED" => Self::Rented,
            "UNDER_CONSTRUCTION" => Self::UnderConstruction,
            "UNRENTED" => Self::Unrented,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyBuildPeriodType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "BEFORE_Y1949" => Self::BeforeY1949,
            "FROM_Y1949_Y1974" => Self::FromY1949Y1974,
            "FROM_Y1975_Y1989" => Self::FromY1975Y1989,
            "FROM_Y1990_Y2005" => Self::FromY1990Y2005,
            "FROM_Y2005" => Self::FromY2005,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyEnergyClass {
    fn from(item: String) -> Self {
        match item.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyGasEmission {
    fn from(item: String) -> Self {
        match item.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyBuildingLegalStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "MONO" => Self::Mono,
            "COPRO" => Self::Copro,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyHabitationUsageType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "HABITATION" => Self::Habitation,
            "MIXTE" => Self::Mixte,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for PropertyUsageType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "COLLECTIVE" => Self::Collective,
            "INDIVIDUAL" => Self::Individual,
            _ => unimplemented!(),
        }
    }
}
