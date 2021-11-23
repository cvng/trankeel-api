use crate::client::Context;
use crate::tenants;
use crate::tenants::CreateTenantPayload;
use crate::tenants::CreateTenantState;
use crate::Command;
use crate::CreateTenantInput;
use crate::Result;
use trankeel_core::database::Db;
use trankeel_data::AuthId;

pub(crate) struct CreateTenant<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> CreateTenant<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

impl<'a> Command for CreateTenant<'a> {
    type Input = CreateTenantInput;
    type Payload = CreateTenantPayload;

    fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = CreateTenantState {
            account: db.accounts().by_auth_id(self.auth_id)?,
            account_owner: db.persons().by_auth_id(self.auth_id)?,
            tenant_identity: None,
        };

        let payload = tenants::create_tenant(state, input)?;

        db.transaction(|| {
            db.persons().create(&payload.tenant_identity)?;
            db.tenants().create(&payload.tenant)?;
            if let Some(warrants) = &payload.warrants {
                db.warrants().create_many(warrants)?;
            }
            if let Some(discussion) = &payload.discussion {
                db.discussions().create(discussion)?;
            }
            Ok(())
        })?;

        Ok(payload)
    }
}
