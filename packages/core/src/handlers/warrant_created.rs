use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::WarrantCreated;

pub fn warrant_created(ctx: &Context, event: WarrantCreated) -> Result<()> {
    let db = ctx.db();

    let WarrantCreated { warrant } = event;

    db.warrants().create(&warrant)?;

    Ok(())
}
