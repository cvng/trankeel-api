use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::PropertyCreated;

pub fn property_created(ctx: &Context, event: PropertyCreated) -> Result<()> {
    let db = ctx.db();

    let PropertyCreated { property } = event;

    db.properties().create(&property)?;

    Ok(())
}
