use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::LenderCreated;

pub fn lender_created(ctx: &Context, event: LenderCreated) -> Result<()> {
    let db = ctx.db();

    let LenderCreated { lender } = event;

    db.lenders().create(&lender)?;

    Ok(())
}
