use super::CreateFurnishedLeaseDetailsInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::AuthId;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::Lease;
use trankeel_data::LeaseData;
use trankeel_data::LeaseId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "LeaseFurnishedUpdateInput")]
pub struct UpdateFurnishedLeaseInput {
    #[graphql(name = "data")]
    details: Option<CreateFurnishedLeaseDetailsInput>,
    file: Option<CreateFileInput>,
    id: LeaseId,
}

// # Operation

pub fn update_furnished_lease(
    db: &impl Db,
    _auth_id: &AuthId,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease> {
    input.validate()?;

    db.leases().update(LeaseData {
        id: input.id,
        details: input.details.map(Into::into),
        account_id: None,
        deposit_amount: None,
        effect_date: None,
        signature_date: None,
        rent_amount: None,
        rent_charges_amount: None,
        type_: None,
        lease_id: None,
        property_id: None,
        expired_at: None,
        renew_date: None,
        duration: None,
    })
}
