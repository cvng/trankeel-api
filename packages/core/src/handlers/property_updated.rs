use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Property;

#[derive(Clone)]
pub struct PropertyUpdated {
    pub property: Property,
}

impl From<PropertyUpdated> for Event {
    fn from(item: PropertyUpdated) -> Self {
        Self::PropertyUpdated(item)
    }
}

pub fn property_updated(ctx: &Context, event: PropertyUpdated) -> Result<()> {
    let db = ctx.db();

    let PropertyUpdated { property } = event;

    db.properties().update(&property)?;

    Ok(())
}
