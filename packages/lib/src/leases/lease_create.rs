use crate::error::Result;
use crate::files::CreateFileInput;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::Amount;
use piteo_data::AuthId;
use piteo_data::DateTime;
use piteo_data::FurnishedLeaseDetails;
use piteo_data::Lease;
use piteo_data::LeaseId;
use piteo_data::LeaseType;
use piteo_data::PropertyId;
use piteo_data::Rent;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;
use piteo_data::TenantStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "LeaseFurnishedInput")]
pub struct CreateFurnishedLeaseInput {
    #[graphql(name = "data")]
    pub details: Option<FurnishedLeaseDetails>,
    pub deposit_amount: Option<Amount>,
    pub effect_date: DateTime,
    pub renew_date: Option<DateTime>,
    pub file: Option<CreateFileInput>,
    pub property_id: PropertyId,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub signature_date: Option<DateTime>,
    pub tenant_ids: Vec<TenantId>,
}

// # Operation

pub fn create_furnished_lease(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease> {
    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;

    // Compute duration.
    let duration = input
        .details
        .clone()
        .and_then(|details| details.duration)
        .unwrap_or_default();

    let lease = db.leases().create(Lease {
        id: LeaseId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        details: input.details.map(Into::into),
        account_id: account.id,
        deposit_amount: input.deposit_amount,
        effect_date: input.effect_date,
        duration,
        signature_date: input.signature_date,
        rent_amount: input.rent_amount,
        rent_charges_amount: input.rent_charges_amount,
        type_: LeaseType::Furnished,
        lease_id: None,
        property_id: input.property_id,
        expired_at: None,
        renew_date: None,
    })?;

    // Generate lease rents.
    add_lease_rents(db, &lease)?;

    // Affect created lease to existing tenants.
    add_lease_tenants(db, lease.id, input.tenant_ids)?;

    Ok(lease)
}

// # Utils

fn add_lease_rents(db: &impl Db, lease: &Lease) -> Result<Vec<Rent>> {
    db.rents().create_many(lease.rents())
}

fn add_lease_tenants(
    db: &impl Db,
    lease_id: LeaseId,
    tenant_ids: Vec<TenantId>,
) -> Result<Vec<Tenant>> {
    tenant_ids
        .iter()
        .map(|&tenant_id| {
            db.tenants().update(TenantData {
                id: tenant_id,
                lease_id: Some(lease_id),
                status: Some(TenantStatus::Uptodate),
                account_id: Default::default(),
                apl: Default::default(),
                birthdate: Default::default(),
                birthplace: Default::default(),
                email: Default::default(),
                first_name: Default::default(),
                last_name: Default::default(),
                note: Default::default(),
                phone_number: Default::default(),
                is_student: Default::default(),
            })
        })
        .collect()
}
