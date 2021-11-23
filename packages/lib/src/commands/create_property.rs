use crate::client::Actor;
use crate::client::Context;
use crate::properties;
use crate::properties::CreatePropertyPayload;
use crate::properties::CreatePropertyState;
use crate::CreatePropertyInput;
use crate::Result;
use trankeel_core::database::Db;

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
