use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::InviteCreated;

pub fn invite_created(ctx: &Context, event: InviteCreated) -> Result<()> {
    let db = ctx.db();

    let InviteCreated { invite } = event;

    db.invites().create(&invite)?;

    Ok(())
}
