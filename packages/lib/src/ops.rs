use crate::client::Actor;
use crate::client::Context;
use crate::messaging;
use crate::messaging::PushMessagePayload;
use crate::messaging::PushMessageState;
use crate::tenants;
use crate::tenants::CreateTenantPayload;
use crate::tenants::CreateTenantState;
use crate::CreateTenantInput;
use crate::PushMessageInput;
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
    };

    let payload = tenants::create_tenant(state, input)?;

    ctx.db().transaction(|| {
        if let Some(person) = &payload.person {
            ctx.db().persons().create(person)?;
        }
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

pub(crate) fn push_message(
    ctx: &Context,
    _actor: &Actor,
    input: PushMessageInput,
) -> Result<PushMessagePayload> {
    let state = PushMessageState {
        discussion: ctx.db().discussions().by_id(&input.discussion_id)?,
    };

    let payload = messaging::push_message(state, input)?;

    ctx.db().transaction(|| {
        ctx.db().messages().create(&payload.message)?;
        ctx.db().discussions().update(&payload.discussion)?;
        Ok(())
    })?;

    Ok(payload)
}
