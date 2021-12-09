use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Advertisement;

#[derive(Clone)]
pub struct AdvertisementUpdated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementUpdated> for Event {
    fn from(item: AdvertisementUpdated) -> Self {
        Event::AdvertisementUpdated(item)
    }
}

pub fn advertisement_updated(ctx: &Context, event: AdvertisementUpdated) -> Result<()> {
    let db = ctx.db();

    let AdvertisementUpdated { advertisement } = event;

    db.advertisements().update(&advertisement)?;

    Ok(())
}
