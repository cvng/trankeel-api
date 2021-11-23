use crate::client::Actor;
use crate::client::Context;
use crate::tenants;
use crate::tenants::UpdateTenantPayload;
use crate::tenants::UpdateTenantState;
use crate::Result;
use crate::UpdateTenantInput;
use trankeel_core::database::Db;

pub(crate) fn update_tenant(
    ctx: &Context,
    _actor: &Actor,
    input: UpdateTenantInput,
) -> Result<UpdateTenantPayload> {
    let state = UpdateTenantState {
        tenant: ctx.db().tenants().by_id(&input.id)?,
    };

    let payload = tenants::update_tenant(state, input)?;

    ctx.db().transaction(|| {
        ctx.db().tenants().update(&payload.tenant)?;
        Ok(())
    })?;

    Ok(payload)
}
