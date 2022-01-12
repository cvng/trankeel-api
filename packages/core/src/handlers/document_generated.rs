use super::receipt_sent_async;
use super::ReceiptSent;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::no;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::templates::LeaseCreatedMail;
use serde_json::Value;
use trankeel_data::Document;
use trankeel_data::File;
use trankeel_data::FileId;
use trankeel_data::FileStatus;
use trankeel_data::FileType;
use trankeel_data::LeaseFile;

#[derive(Clone, Debug)]
pub struct DocumentGenerated {
    pub document: Document,
}

impl DocumentGenerated {
    pub fn with(document: &Document) -> Event {
        Event::DocumentGenerated(Self {
            document: document.clone(),
        })
    }
}

pub fn document_generated(_ctx: &Context, _event: DocumentGenerated) -> Result<()> {
    Ok(())
}

pub async fn document_generated_async(ctx: &Context, event: DocumentGenerated) -> Result<()> {
    let db = ctx.db();

    let DocumentGenerated { document } = event;

    // General processing for any document.
    let payload: Value = serde_json::from_str(&document.payload)?;
    let file_id = payload
        .get("id")
        .ok_or_else(|| no("payload.id"))?
        .as_str()
        .ok_or_else(|| no("payload.id (malformed)"))?
        .parse::<FileId>()?;
    let file = db.files().by_id(&file_id)?;
    let file = db.files().update(&File {
        external_id: Some(document.id),
        status: Some(document.status),
        download_url: Some(document.download_url.clone()),
        preview_url: Some(document.preview_url.clone()),
        ..file
    })?;

    // If not success, stop processing.
    if file.status != Some(FileStatus::Success) {
        return Ok(());
    }

    // Specific processing by document type.
    match file.type_ {
        FileType::LeaseDocument => lease_document_created(ctx, &file).await,
        FileType::RentReceipt => receipt_or_notice_document_created(ctx, &file).await,
        FileType::PaymentNotice => receipt_or_notice_document_created(ctx, &file).await,
    }
}

async fn lease_document_created(ctx: &Context, lease_file: &LeaseFile) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();

    let lease = db.leases().by_lease_file_id(&lease_file.id)?;
    let tenants = db.tenants().by_lease_id(&lease.id)?;

    mailer
        .batch(vec![LeaseCreatedMail::try_new(
            &lease, lease_file, tenants,
        )?])
        .await?;

    Ok(())
}

async fn receipt_or_notice_document_created(ctx: &Context, receipt_or_notice: &File) -> Result<()> {
    let db = ctx.db();

    let rent = match receipt_or_notice.type_ {
        FileType::RentReceipt => db.rents().by_receipt_id(&receipt_or_notice.id)?,
        FileType::PaymentNotice => db.rents().by_notice_id(&receipt_or_notice.id)?,
        _ => unimplemented!(),
    };

    receipt_sent_async(ctx, ReceiptSent { rent_id: rent.id }).await?;

    Ok(())
}
