use async_graphql::SimpleObject;
use trankeel::Amount;
use trankeel::DateTime;
use trankeel::LeaseDuration;
use trankeel::LeaseRentPeriodicity;
use trankeel::LeaseRentReferenceIrl;
use trankeel::RentChargesRecuperationMode;
use trankeel::RentPaymentMethod;

#[derive(SimpleObject)]
pub struct LeaseDetails {
    pub charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    pub charges_revision_method: Option<String>,
    pub colocation_insurance_lender: Option<bool>,
    pub colocation_insurance_monthly_amount: Option<Amount>,
    pub colocation_insurance_total_amount: Option<Amount>,
    pub duration: Option<LeaseDuration>,
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

impl From<trankeel::LeaseDetails> for LeaseDetails {
    fn from(item: trankeel::LeaseDetails) -> Self {
        Self {
            charges_recuperation_mode: item.charges_recuperation_mode,
            charges_revision_method: item.charges_revision_method,
            colocation_insurance_lender: item.colocation_insurance_lender,
            colocation_insurance_monthly_amount: item.colocation_insurance_monthly_amount,
            colocation_insurance_total_amount: item.colocation_insurance_total_amount,
            duration: item.duration,
            lender_fee_cap: item.lender_fee_cap,
            lender_fee_cap_other: item.lender_fee_cap_other,
            lender_fee_cap_prestations: item.lender_fee_cap_prestations,
            other_conditions: item.other_conditions,
            rent_complement: item.rent_complement,
            rent_complement_property_justification: item.rent_complement_property_justification,
            rent_first_amount: item.rent_first_amount,
            rent_irl: item.rent_irl,
            rent_irl_revision_date: item.rent_irl_revision_date,
            rent_maj_decree_increased_amount: item.rent_maj_decree_increased_amount,
            rent_maj_decree_reference_amount: item.rent_maj_decree_reference_amount,
            rent_majoration_decree: item.rent_majoration_decree,
            rent_max_evolution_relocation: item.rent_max_evolution_relocation,
            rent_payment_date: item.rent_payment_date,
            rent_payment_method: item.rent_payment_method,
            rent_payment_place: item.rent_payment_place,
            rent_periodicity: item.rent_periodicity,
            rent_underestimated_method: item.rent_underestimated_method,
            rent_underestimated_monthly_variation: item.rent_underestimated_monthly_variation,
            resolutary_clause: item.resolutary_clause,
            solidarity_clause: item.solidarity_clause,
            tenant_fee_cap_new_rental: item.tenant_fee_cap_new_rental,
            tenant_fee_cap_prestations: item.tenant_fee_cap_prestations,
            tenant_fee_cap_report_by_meter: item.tenant_fee_cap_report_by_meter,
            tenant_fee_cap_report_prestations: item.tenant_fee_cap_report_prestations,
            tenant_last_rent_amount: item.tenant_last_rent_amount,
            tenant_last_rent_received_date: item.tenant_last_rent_received_date,
            tenant_last_rent_revision_date: item.tenant_last_rent_revision_date,
            works_decence_since_last_rental: item.works_decence_since_last_rental,
            works_rent_decrease_tenant: item.works_rent_decrease_tenant,
            works_rent_increase_lender: item.works_rent_increase_lender,
        }
    }
}
