use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::LeaseUpdated;

pub fn lease_updated(ctx: &Context, event: LeaseUpdated) -> Result<()> {
    let db = ctx.db();

    let LeaseUpdated { lease } = event;

    db.leases().update(&lease)?;

    Ok(())
}
