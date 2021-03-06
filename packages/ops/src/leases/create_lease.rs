use crate::command::Command;
use crate::error::DomainError;
use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseAffected;
use crate::event::LeaseCreated;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::Lease;
use trankeel_data::LeaseDetails;
use trankeel_data::LeaseDuration;
use trankeel_data::LeaseId;
use trankeel_data::LeaseRentPeriodicity;
use trankeel_data::LeaseRentReferenceIrl;
use trankeel_data::LeaseType;
use trankeel_data::PropertyId;
use trankeel_data::RentChargesRecuperationMode;
use trankeel_data::RentPaymentMethod;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct LeaseDetailsInput {
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

#[derive(InputObject, Validate)]
pub struct CreateLeaseInput {
    pub details: Option<LeaseDetailsInput>,
    pub deposit_amount: Option<Amount>,
    pub effect_date: DateTime,
    pub renew_date: Option<DateTime>,
    pub type_: LeaseType,
    pub property_id: PropertyId,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub signature_date: Option<DateTime>,
    pub tenant_ids: Vec<TenantId>,
}

pub struct CreateLease {
    lease_id: LeaseId,
    account: Account,
    tenants: Vec<Tenant>,
}

impl CreateLease {
    pub fn new(lease_id: LeaseId, account: &Account, tenants: &[Tenant]) -> Self {
        Self {
            lease_id,
            account: account.clone(),
            tenants: tenants.to_vec(),
        }
    }
}

impl Command for CreateLease {
    type Input = CreateLeaseInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self {
            lease_id,
            account,
            tenants,
        } = self;

        // Check signature date.
        if let Some(signature_date) = input.signature_date {
            if input.effect_date.inner() > signature_date.inner() {
                return Err(DomainError::InvalidSignatureDate(signature_date).into());
            }
        }

        // Compute duration.
        let duration = input
            .details
            .as_ref()
            .and_then(|details| details.duration)
            .unwrap_or_default();

        // Create lease.
        let lease = Lease {
            id: lease_id,
            created_at: Default::default(),
            updated_at: Default::default(),
            details: input.details.map(Into::into),
            account_id: account.id,
            deposit_amount: input.deposit_amount.unwrap_or_default(),
            effect_date: input.effect_date,
            duration,
            signature_date: input.signature_date,
            rent_amount: input.rent_amount,
            rent_charges_amount: input.rent_charges_amount,
            type_: input.type_,
            lease_id: None,
            property_id: input.property_id,
            expired_at: None,
            renew_date: input.renew_date,
        };

        // Make the lease active by using a signature date.
        let lease = Lease {
            signature_date: lease.signature_date.or(Some(lease.effect_date)),
            ..lease
        };

        // Generate rents.
        let rents = lease.rents();

        // Update status for existing tenants.
        let tenants = tenants.into_iter().map(|tenant| Tenant {
            lease_id: Some(lease.id),
            ..tenant
        });

        Ok(vec![LeaseCreated {
            lease: lease.clone(),
            rents,
        }
        .into()]
        .into_iter()
        .chain(
            tenants
                .into_iter()
                .map(|tenant| {
                    LeaseAffected {
                        lease_id: lease.id,
                        tenant_id: tenant.id,
                    }
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .collect())
    }
}

impl From<LeaseDetailsInput> for LeaseDetails {
    fn from(item: LeaseDetailsInput) -> Self {
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
