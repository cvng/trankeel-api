use crate::Amount;
use crate::DateTime;
use async_graphql::Enum;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseFurnishedDuration {
    NineMonths,
    OneYear,
}

/// https://www.service-public.fr/particuliers/vosdroits/F13723
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseRentReferenceIrl {
    AprilFirstSemesterY2021,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseRentPeriodicity {
    Annualy,
    Monthly,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RentChargesRecuperationMode {
    Package,
    Periodic,
    Reel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RentPaymentMethod {
    After,
    Before,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseNakedDuration {
    ThreeYears,
    SixYears,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSqlRow)]
#[serde(rename_all = "camelCase")]
pub struct LeaseFurnishedData {
    pub charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    pub charges_revision_method: Option<String>,
    pub colocation_insurance_lender: Option<bool>,
    pub colocation_insurance_monthly_amount: Option<Amount>,
    pub colocation_insurance_total_amount: Option<Amount>,
    pub duration: Option<LeaseFurnishedDuration>,
    pub lender_fee_cap: Option<Amount>,
    pub lender_fee_cap_other: Option<String>,
    pub lender_fee_cap_prestations: Option<Amount>,
    pub other_conditions: Option<String>,
    pub rent_complement: Option<Amount>,
    pub rent_complement_property_justification: Option<String>,
    pub rent_first_amount: Option<Amount>,
    pub rent_irl: Option<LeaseRentReferenceIrl>,
    pub rent_irl_revision_date: Option<String>,
    pub rent_maj_decree_increased_amount: Option<Amount>,
    pub rent_maj_decree_reference_amount: Option<Amount>,
    pub rent_majoration_decree: Option<bool>,
    pub rent_max_evolution_relocation: Option<bool>,
    pub rent_payment_date: Option<DateTime>,
    pub rent_payment_method: Option<RentPaymentMethod>,
    pub rent_payment_place: Option<String>,
    pub rent_periodicity: Option<LeaseRentPeriodicity>,
    pub rent_underestimated_method: Option<String>,
    pub rent_underestimated_monthly_variation: Option<Amount>,
    pub resolutary_clause: Option<String>,
    pub solidarity_clause: Option<String>,
    pub tenant_fee_cap_new_rental: Option<Amount>,
    pub tenant_fee_cap_prestations: Option<Amount>,
    pub tenant_fee_cap_report_by_meter: Option<Amount>,
    pub tenant_fee_cap_report_prestations: Option<Amount>,
    pub tenant_last_rent_amount: Option<Amount>,
    pub tenant_last_rent_received_date: Option<String>,
    pub tenant_last_rent_revision_date: Option<String>,
    pub works_decence_since_last_rental: Option<String>,
    pub works_rent_decrease_tenant: Option<String>,
    pub works_rent_increase_lender: Option<String>,
}

// # Impls

impl FromSql<Jsonb, Pg> for LeaseFurnishedData {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
