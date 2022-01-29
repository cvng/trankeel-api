use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::LeaseDeleted;

pub fn lease_deleted(ctx: &Context, event: LeaseDeleted) -> Result<()> {
    let db = ctx.db();

    let LeaseDeleted { lease_id } = event;

    db.leases().delete(&lease_id)?;

    Ok(())
}
