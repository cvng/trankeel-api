use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::AdvertisementCreated;
use trankeel_ops::properties;
use trankeel_ops::state::State;

pub fn advertisement_created(ctx: &Context, event: AdvertisementCreated) -> Result<()> {
    let db = ctx.db();

    let state = State {
        advertisements: Vec::new(),
        ..Default::default()
    };

    let State { advertisements, .. } = properties::advertisement_created(state, event);

    if let Some(advertisement) = advertisements.first() {
        db.advertisements().create(advertisement)?;
    }

    Ok(())
}
