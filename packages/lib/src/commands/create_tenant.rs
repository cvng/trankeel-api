use crate::client::Actor;
use crate::client::Context;
use crate::tenants;
use crate::tenants::CreateTenantPayload;
use crate::tenants::CreateTenantState;
use crate::CreateTenantInput;
use crate::Result;
use trankeel_core::database::Db;

pub(crate) fn create_tenant(
    ctx: &Context,
    actor: &Actor,
    input: CreateTenantInput,
) -> Result<CreateTenantPayload> {
    let state = CreateTenantState {
        account: ctx.db().accounts().by_auth_id(actor.check()?)?,
        account_owner: ctx.db().persons().by_auth_id(actor.check()?)?,
        tenant_identity: None,
    };

    let payload = tenants::create_tenant(state, input)?;

    ctx.db().transaction(|| {
        ctx.db().persons().create(&payload.tenant_identity)?;
        ctx.db().tenants().create(&payload.tenant)?;
        if let Some(warrants) = &payload.warrants {
            for warrant in warrants {
                ctx.db().warrants().create(warrant)?;
            }
        }
        if let Some(discussion) = &payload.discussion {
            ctx.db().discussions().create(discussion)?;
        }
        Ok(())
    })?;

    Ok(payload)
}
