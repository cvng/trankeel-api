use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::PropertyDeleted;

pub fn property_deleted(ctx: &Context, event: PropertyDeleted) -> Result<()> {
    let db = ctx.db();

    let PropertyDeleted { property_id } = event;

    db.properties().delete(&property_id)?;

    Ok(())
}
