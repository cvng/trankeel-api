use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::LeaseCreated;

pub fn lease_created(ctx: &Context, event: LeaseCreated) -> Result<()> {
    let db = ctx.db();

    let LeaseCreated { lease, rents } = event;

    db.leases().create(&lease)?;
    db.rents().create_many(&rents)?;

    Ok(())
}
