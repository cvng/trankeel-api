use crate::client::Context;
use crate::leases;
use crate::leases::AddExistingLeaseState;
use crate::AddExistingLeaseInput;
use crate::AddExistingLeasePayload;
use crate::Command;
use crate::Result;
use trankeel_core::database::Db;
use trankeel_data::AuthId;

pub(crate) struct AddExistingLease<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> AddExistingLease<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

impl<'a> Command for AddExistingLease<'a> {
    type Input = AddExistingLeaseInput;
    type Payload = AddExistingLeasePayload;

    fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = AddExistingLeaseState {
            account: db.accounts().by_auth_id(self.auth_id)?,
            account_owner: db.persons().by_auth_id(self.auth_id)?,
        };

        let payload = leases::add_existing_lease(state, input)?;

        db.transaction(|| {
            db.properties().create(&payload.property)?;
            db.leases().create(&payload.lease)?;
            db.rents().create_many(&payload.rents)?;
            db.persons().create_many(&payload.identities)?;
            db.tenants().create_many(&payload.tenants)?;
            db.discussions().create_many(&payload.discussions)?;
            Ok(())
        })?;

        Ok(payload)
    }
}
