use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::AdvertisementUpdated;

pub fn advertisement_updated(ctx: &Context, event: AdvertisementUpdated) -> Result<()> {
    let db = ctx.db();

    let AdvertisementUpdated { advertisement } = event;

    db.advertisements().update(&advertisement)?;

    Ok(())
}
