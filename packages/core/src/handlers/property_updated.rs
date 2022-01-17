use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::PropertyUpdated;

pub fn property_updated(ctx: &Context, event: PropertyUpdated) -> Result<()> {
    let db = ctx.db();

    let PropertyUpdated { property } = event;

    db.properties().update(&property)?;

    Ok(())
}
