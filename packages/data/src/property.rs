use crate::Address;
use crate::Id;
use async_graphql::Enum;

// # Types

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyRoomType {
    Other,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyStatus {
    ForSale,
    Inactive,
    Rented,
    UnderConstruction,
    Unrented,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyBuildPeriodType {
    BeforeY1949,
    FromY1949Y1974,
    FromY1975Y1989,
    FromY1990Y2005,
    FromY2005,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyEnergyClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyGasEmission {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyBuildingLegalStatus {
    Copro,
    Mono,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyHabitationUsageType {
    Habitation,
    Mixte,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum PropertyUsageType {
    Collective,
    Individual,
}

#[derive(Clone, Queryable)]
pub struct Property {
    pub account_id: Option<Id>,
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
    pub id: Id,
    pub lender_id: Id,
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
