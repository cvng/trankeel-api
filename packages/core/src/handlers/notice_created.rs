use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::pdfmaker::Pdfmaker;
use crate::templates::NoticeDocument;
use chrono::Utc;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Notice;
use trankeel_data::Rent;

#[derive(Clone)]
pub struct NoticeCreated {
    pub notice: Notice,
    pub rent: Rent,
}

impl From<NoticeCreated> for Event {
    fn from(item: NoticeCreated) -> Self {
        Event::NoticeCreated(item)
    }
}

pub fn notice_created(ctx: &Context, event: NoticeCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let NoticeCreated { notice, rent } = event;

    db.files().create(&notice)?;
    db.rents().update(&rent)?;

    let participant = db.persons().by_notice_id(&notice.id)?;

    messenger.message(
        EventType::NoticeCreated,
        Eventable::File(notice),
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

pub async fn notice_created_async(ctx: &Context, event: NoticeCreated) -> Result<()> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();

    let NoticeCreated { notice, rent } = event;

    let lease = db.leases().by_id(&rent.lease_id)?;
    let tenants = db.tenants().by_lease_id(&lease.id)?;
    let property = db.properties().by_id(&lease.property_id)?;
    let lender = db.lenders().by_id(&property.lender_id)?;

    // Try to generate notice document (PDF).
    let document = pdfmaker
        .generate(NoticeDocument::try_new(
            notice.clone(),
            rent.clone(),
            lender,
            tenants,
            property,
            Utc::now().into(),
        )?)
        .await?;

    db.files().update(&Notice {
        external_id: Some(document.id),
        ..notice
    })?;

    Ok(())
}
