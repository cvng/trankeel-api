use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use serde::Deserialize;
use serde::Serialize;

// # Models

#[derive(Clone, Serialize, Deserialize, Debug, FromSqlRow)]
#[serde(rename_all = "camelCase")]
pub struct LeaseData {
    pub charges_recuperation_mode: Option<String>, // Option<RentChargesRecuperationMode>,
    pub charges_revision_method: Option<String>,
    pub colocation_insurance_lender: Option<bool>,
    pub colocation_insurance_monthly_amount: Option<decimal::Decimal>,
    pub colocation_insurance_total_amount: Option<decimal::Decimal>,
    pub duration: Option<String>, // Option<LeaseFurnishedDuration>,
    pub lender_fee_cap: Option<decimal::Decimal>,
    pub lender_fee_cap_other: Option<String>,
    pub lender_fee_cap_prestations: Option<decimal::Decimal>,
    pub other_conditions: Option<String>,
    pub rent_complement: Option<decimal::Decimal>,
    pub rent_complement_property_justification: Option<String>,
    pub rent_first_amount: Option<decimal::Decimal>,
    pub rent_irl: Option<String>, // Option<LeaseRentReferenceIrl>,
    pub rent_irl_revision_date: Option<String>,
    pub rent_maj_decree_increased_amount: Option<decimal::Decimal>,
    pub rent_maj_decree_reference_amount: Option<decimal::Decimal>,
    pub rent_majoration_decree: Option<bool>,
    pub rent_max_evolution_relocation: Option<bool>,
    pub rent_payment_date: Option<chrono::NaiveDateTime>,
    pub rent_payment_method: Option<String>, // Option<RentPaymentMethod>,
    pub rent_payment_place: Option<String>,
    pub rent_periodicity: Option<String>, // Option<LeaseRentPeriodicity>,
    pub rent_underestimated_method: Option<String>,
    pub rent_underestimated_monthly_variation: Option<decimal::Decimal>,
    pub resolutary_clause: Option<String>,
    pub solidarity_clause: Option<String>,
    pub tenant_fee_cap_new_rental: Option<decimal::Decimal>,
    pub tenant_fee_cap_prestations: Option<decimal::Decimal>,
    pub tenant_fee_cap_report_by_meter: Option<decimal::Decimal>,
    pub tenant_fee_cap_report_prestations: Option<decimal::Decimal>,
    pub tenant_last_rent_amount: Option<decimal::Decimal>,
    pub tenant_last_rent_received_date: Option<String>,
    pub tenant_last_rent_revision_date: Option<String>,
    pub works_decence_since_last_rental: Option<String>,
    pub works_rent_decrease_tenant: Option<String>,
    pub works_rent_increase_lender: Option<String>,
}

impl FromSql<Jsonb, Pg> for LeaseData {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
