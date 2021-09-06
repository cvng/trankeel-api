use crate::database::Db;
use crate::files::FileInput;
use async_graphql::InputObject;
use eyre::Error;
use piteo_data::Amount;
use piteo_data::AuthId;
use piteo_data::DateTime;
use piteo_data::FurnishedLeaseDetails;
use piteo_data::Lease;
use piteo_data::LeaseTenant;
use piteo_data::LeaseType;
use piteo_data::PropertyId;
use piteo_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "LeaseFurnishedInput")]
pub struct CreateFurnishedLeaseInput {
    #[graphql(name = "data")]
    details: Option<FurnishedLeaseDetails>,
    deposit_amount: Option<Amount>,
    effect_date: DateTime,
    renew_date: Option<DateTime>,
    file: Option<FileInput>,
    property_id: PropertyId,
    rent_amount: Amount,
    rent_charges_amount: Option<Amount>,
    signature_date: Option<DateTime>,
    tenant_ids: Vec<TenantId>,
}

// # Operation

pub fn create_furnished_lease(
    db: impl Db,
    auth_id: AuthId,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;

    // Compute duration.
    let duration = input
        .details
        .clone()
        .and_then(|details| details.duration)
        .unwrap_or_default();

    // Compute renew date.
    let renew_date = input
        .renew_date
        .unwrap_or_else(|| input.effect_date.shift_months(duration.in_months()));

    let lease = db.leases().create(Lease {
        account_id: account.id,
        deposit_amount: input.deposit_amount,
        effect_date: input.effect_date,
        renew_date: Some(renew_date),
        signature_date: input.signature_date,
        rent_amount: input.rent_amount,
        rent_charges_amount: input.rent_charges_amount,
        type_: LeaseType::Furnished,
        lease_id: None,
        property_id: input.property_id,
        id: Default::default(),
        details: input.details.map(Into::into),
        expired_at: None,
    })?;

    // Affect created lease to existing tenants.
    for tenant_id in input.tenant_ids {
        db.lease_tenants().create(LeaseTenant {
            lease_id: lease.id,
            tenant_id,
        })?;
    }

    Ok(lease)
}
