use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::AdvertisementCreated;

pub fn advertisement_created(ctx: &Context, event: AdvertisementCreated) -> Result<()> {
    let db = ctx.db();

    let AdvertisementCreated { advertisement } = event;

    db.advertisements().create(&advertisement)?;

    Ok(())
}
