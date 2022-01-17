use crate::sql_schema::properties;
use crate::AccountId;
use crate::Address;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LenderId;

// # Types

pub type PropertyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertyroomtype"]
pub enum PropertyRoomType {
    Other,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertybuildperiodtype"]
pub enum PropertyBuildPeriodType {
    #[graphql(name = "BEFORE_Y1949")]
    Before_1949,
    #[graphql(name = "FROM_Y1949_Y1974")]
    From_1949_1974,
    #[graphql(name = "FROM_Y1975_Y1989")]
    From_1975_1989,
    #[graphql(name = "FROM_Y1990_Y2005")]
    From_1990_2005,
    #[graphql(name = "FROM_Y2005")]
    From_2005,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertyenergyclass"]
pub enum PropertyEnergyClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertygasemission"]
pub enum PropertyGasEmission {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertybuildinglegalstatus"]
pub enum PropertyBuildingLegalStatus {
    Copro,
    Mono,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertyhabitationusagetype"]
pub enum PropertyHabitationUsageType {
    Habitation,
    Mixte,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, DbEnum, Enum)]
#[DieselType = "Propertyusagetype"]
pub enum PropertyUsageType {
    Collective,
    Individual,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Propertystatus"]
pub enum PropertyStatus {
    ForSale,
    Inactive,
    Rented,
    UnderConstruction,
    Unrented,
}

impl Default for PropertyStatus {
    fn default() -> Self {
        Self::Rented
    }
}

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
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
    pub room_count: Option<PropertyRoomType>,
    pub status: PropertyStatus,
    pub surface: Option<f32>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
    pub lender_id: LenderId,
}
