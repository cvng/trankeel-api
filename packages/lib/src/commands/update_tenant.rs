use crate::client::Context;
use crate::tenants;
use crate::tenants::UpdateTenantPayload;
use crate::tenants::UpdateTenantState;
use crate::Command;
use crate::Result;
use crate::UpdateTenantInput;
use trankeel_core::database::Db;

pub(crate) struct UpdateTenant<'a> {
    context: &'a Context,
}

impl<'a> UpdateTenant<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<'a> Command for UpdateTenant<'a> {
    type Input = UpdateTenantInput;
    type Payload = UpdateTenantPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = UpdateTenantState {
            tenant: db.tenants().by_id(&input.id)?,
        };

        let payload = tenants::update_tenant(state, input)?;

        db.transaction(|| {
            db.tenants().update(&payload.tenant)?;
            Ok(())
        })?;

        Ok(payload)
    }
}
