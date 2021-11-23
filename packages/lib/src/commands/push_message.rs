use crate::client::Actor;
use crate::client::Context;
use crate::messaging;
use crate::messaging::PushMessagePayload;
use crate::messaging::PushMessageState;
use crate::PushMessageInput;
use crate::Result;
use trankeel_core::database::Db;

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
