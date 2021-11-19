use crate::client::Actor;
use crate::client::Context;
use crate::leases::AddExistingLease;
use crate::leases::AddExistingLeaseState;
use crate::messaging;
use crate::messaging::PushMessagePayload;
use crate::messaging::PushMessageState;
use crate::properties;
use crate::properties::CreatePropertyPayload;
use crate::properties::CreatePropertyState;
use crate::tenants;
use crate::tenants::CreateTenantPayload;
use crate::tenants::CreateTenantState;
use crate::tenants::UpdateTenantPayload;
use crate::tenants::UpdateTenantState;
use crate::AddExistingLeaseInput;
use crate::AddExistingLeasePayload;
use crate::Command;
use crate::CreatePropertyInput;
use crate::CreateTenantInput;
use crate::PushMessageInput;
use crate::Result;
use crate::UpdateTenantInput;
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
        ctx.db().persons().create(&payload.tenant.1)?;
        ctx.db().tenants().create(&payload.tenant.0)?;
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

pub(crate) fn add_existing_lease(
    ctx: &Context,
    actor: &Actor,
    input: AddExistingLeaseInput,
) -> Result<AddExistingLeasePayload> {
    let state = AddExistingLeaseState {
        account: ctx.db().accounts().by_auth_id(actor.check()?)?,
        account_owner: ctx.db().persons().by_auth_id(actor.check()?)?,
    };

    let payload = AddExistingLease::run(state, input)?;

    ctx.db().transaction(|| {
        ctx.db().properties().create(&payload.property)?;
        ctx.db().leases().create(&payload.lease)?;
        ctx.db().rents().create_many(&payload.rents)?;
        for tenant in &payload.tenants {
            ctx.db().persons().create(&tenant.1)?;
            ctx.db().tenants().create(&tenant.0)?;
        }
        if let Some(discussions) = &payload.discussions {
            for discussion in discussions {
                ctx.db().discussions().create(discussion)?;
            }
        }
        Ok(())
    })?;

    Ok(payload)
}

pub(crate) fn create_property(
    ctx: &Context,
    actor: &Actor,
    input: CreatePropertyInput,
) -> Result<CreatePropertyPayload> {
    let state = CreatePropertyState {
        account: ctx.db().accounts().by_auth_id(actor.check()?)?,
    };

    let payload = properties::create_property(state, input)?;

    ctx.db().transaction(|| {
        ctx.db().properties().create(&payload.property)?;
        Ok(())
    })?;

    Ok(payload)
}
