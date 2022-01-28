use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::pdfmaker::Pdfmaker;
use crate::templates::LeaseDocument;
use chrono::Utc;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_ops::event::LeaseFileRequested;

pub fn lease_file_requested(_ctx: &Context, _event: LeaseFileRequested) -> Result<()> {
    Ok(())
}

pub async fn lease_file_requested_async(ctx: &Context, event: LeaseFileRequested) -> Result<()> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();

    let LeaseFileRequested { lease_id } = event;

    let lease = db.leases().by_id(&lease_id)?;

    // Create lease file.
    let lease_file = LeaseFile::lease_document(&lease);

    // Link lease file with lease.
    let lease = Lease {
        id: lease.id,
        lease_id: Some(lease_file.id),
        ..lease
    };

    db.files().create(&lease_file)?;
    db.leases().update(&lease)?;

    // Generate lease document (PDF) and assign document external ID to lease file.
    let document = pdfmaker
        .generate(LeaseDocument::try_new(
            &lease,
            &lease_file,
            Utc::now().into(),
        )?)
        .await?;

    db.files().update(&LeaseFile {
        external_id: Some(document.id),
        ..lease_file
    })?;

    Ok(())
}
