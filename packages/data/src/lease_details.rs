use crate::Amount;
use crate::DateTime;
use async_graphql::Enum;
use async_graphql::InputObject;
use diesel_as_jsonb::AsJsonb;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
#[graphql(name = "LeaseFurnishedDuration")]
pub enum FurnishedLeaseDuration {
    NineMonths,
    OneYear,
}

impl FurnishedLeaseDuration {
    pub fn in_months(&self) -> i32 {
        match self {
            Self::NineMonths => 9,
            Self::OneYear => 12,
        }
    }
}

impl Default for FurnishedLeaseDuration {
    fn default() -> Self {
        Self::OneYear
    }
}

/// https://www.service-public.fr/particuliers/vosdroits/F13723
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum LeaseRentReferenceIrl {
    AprilFirstSemesterY2021,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum LeaseRentPeriodicity {
    Annualy,
    Monthly,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum RentChargesRecuperationMode {
    Package,
    Periodic,
    Reel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum RentPaymentMethod {
    After,
    Before,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum LeaseNakedDuration {
    ThreeYears,
    SixYears,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, AsJsonb, InputObject)]
#[serde(rename_all = "camelCase")]
#[graphql(name = "LeaseFurnishedDataInput")]
pub struct FurnishedLeaseDetails {
    pub charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    pub charges_revision_method: Option<String>,
    pub colocation_insurance_lender: Option<bool>,
    pub colocation_insurance_monthly_amount: Option<Amount>,
    pub colocation_insurance_total_amount: Option<Amount>,
    pub duration: Option<FurnishedLeaseDuration>,
    pub lender_fee_cap: Option<Amount>,
    pub lender_fee_cap_other: Option<String>,
    pub lender_fee_cap_prestations: Option<Amount>,
    pub other_conditions: Option<String>,
    pub rent_complement: Option<Amount>,
    pub rent_complement_property_justification: Option<String>,
    pub rent_first_amount: Option<Amount>,
    pub rent_irl: Option<LeaseRentReferenceIrl>,
    pub rent_irl_revision_date: Option<DateTime>,
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
    pub tenant_last_rent_received_date: Option<DateTime>,
    pub tenant_last_rent_revision_date: Option<DateTime>,
    pub works_decence_since_last_rental: Option<String>,
    pub works_rent_decrease_tenant: Option<String>,
    pub works_rent_increase_lender: Option<String>,
}
