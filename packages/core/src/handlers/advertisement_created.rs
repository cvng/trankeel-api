use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Advertisement;

#[derive(Clone)]
pub struct AdvertisementCreated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementCreated> for Event {
    fn from(item: AdvertisementCreated) -> Self {
        Event::AdvertisementCreated(item)
    }
}

pub fn advertisement_created(ctx: &Context, event: AdvertisementCreated) -> Result<()> {
    let db = ctx.db();

    let AdvertisementCreated { advertisement } = event;

    db.advertisements().create(&advertisement)?;

    Ok(())
}
