use super::receipt_sent_async;
use super::ReceiptSent;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::templates::LeaseCreatedMail;
use trankeel_data::Document;
use trankeel_data::File;
use trankeel_data::FileStatus;
use trankeel_data::FileType;
use trankeel_data::LeaseFile;

#[derive(Clone, Debug)]
pub struct DocumentGenerated {
    pub document: Document,
}

impl From<DocumentGenerated> for Event {
    fn from(item: DocumentGenerated) -> Self {
        Self::DocumentGenerated(item)
    }
}

pub fn document_generated(_ctx: &Context, _event: DocumentGenerated) -> Result<()> {
    Ok(())
}

pub async fn document_generated_async(ctx: &Context, event: DocumentGenerated) -> Result<()> {
    println!("Document generated: {:?}", event);

    let db = ctx.db();

    let DocumentGenerated { document } = event;

    // General processing for any document.
    let file = db.files().by_external_id(&document.id)?;
    let file = db.files().update(&File {
        id: file.id,
        status: Some(document.status),
        download_url: document.download_url.clone(),
        preview_url: Some(document.preview_url.clone()),
        ..file
    })?;

    // If not success, stop processing.
    if file.status != Some(FileStatus::Success) {
        println!("Document errors: {:?}", document.errors);
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
