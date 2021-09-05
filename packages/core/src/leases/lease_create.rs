use crate::files::FileInput;
use async_graphql::InputObject;
use piteo_data::Amount;
use piteo_data::DateTime;
use piteo_data::LeaseFurnishedDuration;
use piteo_data::LeaseRentPeriodicity;
use piteo_data::LeaseRentReferenceIrl;
use piteo_data::PropertyId;
use piteo_data::RentChargesRecuperationMode;
use piteo_data::RentPaymentMethod;
use piteo_data::TenantId;

#[derive(InputObject)]
pub struct LeaseFurnishedInput {
    data: Option<LeaseFurnishedDataInput>,
    deposit_amount: Option<Amount>,
    effect_date: DateTime,
    renew_date: Option<DateTime>,
    file: Option<FileInput>,
    property_id: PropertyId,
    rent_amount: Amount,
    rent_charges_amount: Option<Amount>,
    signature_date: Option<DateTime>,
    tenant_ids: Vec<TenantId>,
    // r#type: LeaseType,
}

#[derive(InputObject)]
pub struct LeaseFurnishedDataInput {
    charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    charges_revision_method: Option<String>,
    colocation_insurance_lender: Option<bool>,
    colocation_insurance_monthly_amount: Option<Amount>,
    colocation_insurance_total_amount: Option<Amount>,
    duration: Option<LeaseFurnishedDuration>,
    lender_fee_cap: Option<Amount>,
    lender_fee_cap_other: Option<String>,
    lender_fee_cap_prestations: Option<Amount>,
    other_conditions: Option<String>,
    rent_complement: Option<Amount>,
    rent_complement_property_justification: Option<String>,
    rent_first_amount: Option<Amount>,
    rent_irl: Option<LeaseRentReferenceIrl>,
    rent_irl_revision_date: Option<DateTime>,
    rent_maj_decree_increased_amount: Option<Amount>,
    rent_maj_decree_reference_amount: Option<Amount>,
    rent_majoration_decree: Option<bool>,
    rent_max_evolution_relocation: Option<bool>,
    rent_payment_date: Option<DateTime>,
    rent_payment_method: Option<RentPaymentMethod>,
    rent_payment_place: Option<String>,
    rent_periodicity: Option<LeaseRentPeriodicity>,
    rent_underestimated_method: Option<String>,
    rent_underestimated_monthly_variation: Option<Amount>,
    resolutary_clause: Option<String>,
    solidarity_clause: Option<String>,
    tenant_fee_cap_new_rental: Option<Amount>,
    tenant_fee_cap_prestations: Option<Amount>,
    tenant_fee_cap_report_by_meter: Option<Amount>,
    tenant_fee_cap_report_prestations: Option<Amount>,
    tenant_last_rent_amount: Option<Amount>,
    tenant_last_rent_received_date: Option<DateTime>,
    tenant_last_rent_revision_date: Option<DateTime>,
    works_decence_since_last_rental: Option<String>,
    works_rent_decrease_tenant: Option<String>,
    works_rent_increase_lender: Option<String>,
}
