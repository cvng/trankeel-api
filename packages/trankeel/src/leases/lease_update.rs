use super::CreateFurnishedLeaseDetailsInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::AuthId;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::Lease;
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

    let lease = db.leases().by_id(&input.id)?;

    db.leases().update(&Lease {
        id: input.id,
        details: input.details.map(Into::into),
        ..lease
    })
}
