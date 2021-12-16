use crate::error::Result;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher;
use trankeel_core::handlers::NoticeCreated;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_core::templates::NoticeDocument;
use trankeel_data::notice_filename;
use trankeel_data::DateTime;
use trankeel_data::FileType;
use trankeel_data::Notice;
use trankeel_data::NoticeId;
use trankeel_data::Rent;
use trankeel_data::RentId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateNoticesInput {
    rent_ids: Vec<RentId>,
    date: Option<DateTime>,
}

pub struct CreateNoticesPayload {
    pub notices: Vec<Notice>,
}

pub(crate) struct CreateNotices<'a> {
    ctx: &'a Context,
}

impl<'a> CreateNotices<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { ctx }
    }
}

impl<'a> CreateNotices<'a> {
    pub async fn run(self, input: CreateNoticesInput) -> Result<CreateNoticesPayload> {
        let ctx = self.ctx;
        let db = self.ctx.db();
        let pdfmaker = self.ctx.pdfmaker();

        input.validate()?;

        // Find rents.
        let mut rents = vec![];

        for rent_id in input.rent_ids {
            let rent = db.rents().by_id(&rent_id)?;
            rents.push(rent);
        }

        // Generate notices.
        let mut notices = vec![];

        for rent in rents {
            // Try to fetch associated entities.
            let lease = db.leases().by_id(&rent.lease_id)?;
            let tenants = db.tenants().by_lease_id(&lease.id)?;
            let property = db.properties().by_id(&lease.property_id)?;
            let lender = db.lenders().by_id(&property.lender_id)?;

            // Init new notice.
            let notice_id = NoticeId::new();
            let mut notice = Notice {
                id: notice_id,
                type_: FileType::PaymentNotice,
                filename: Some(notice_filename(&notice_id, &rent)),
                status: None,
                external_id: None,
                download_url: None,
                preview_url: None,
                created_at: None,
                updated_at: None,
            };

            // Try to generate notice document (PDF).
            let document = NoticeDocument::try_new(
                notice.clone(),
                rent.clone(),
                lender,
                tenants,
                property,
                Utc::now().into(),
            )?;
            let document = pdfmaker.generate(document).await?;

            // Assign notice external ID.
            notice.external_id = Some(document.id);
            notice.status = Some(document.status);

            // Create notice.
            let notice = match db.files().create(&notice) {
                Ok(notice) => notice,
                Err(err) => return Err(err),
            };

            // Link notice with rent.
            db.rents().update(&Rent {
                id: rent.id,
                notice_id: Some(notice.id),
                ..rent
            })?;

            notices.push(notice.clone());

            dispatcher::dispatch(ctx, vec![NoticeCreated { notice }.into()]).await?;
        }

        Ok(CreateNoticesPayload { notices })
    }
}
