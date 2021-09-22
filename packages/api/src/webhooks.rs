use piteo::database::Db;
use piteo::leases::SendReceiptsInput;
use piteo::pdfmaker::Document;
use piteo::DbPool;
use piteo::FileData;
use piteo::FileStatus;
use piteo::FileType;
use piteo::Pg;
use piteo::Provider;
use piteo::Receipt;
use rocket::http::Status;
use rocket::info;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::warn;

/// https://www.pdfmonkey.io/fr/doc/api/webhooks
#[derive(Debug, Deserialize)]
pub struct PdfmonkeyPayload {
    document: Document,
}

#[post("/webhooks/pdfmonkey", data = "<request>", format = "application/json")]
pub async fn pdfmonkey_request(request: Json<PdfmonkeyPayload>) -> Status {
    info!("Received pdfmonkey request: {:?}", request);

    let db_pool = Pg::init().inner();
    let db = piteo::db(&db_pool);

    let document = request.document.clone();

    // General processing for any document.
    let file = db.files().by_external_id(&document.id).unwrap();
    let file = db
        .files()
        .update(FileData {
            id: file.id,
            status: Some(document.status),
            download_url: document.download_url.clone(),
            preview_url: Some(document.preview_url.clone()),
            ..Default::default()
        })
        .unwrap();

    // If not success, stop processing.
    if file.status != Some(FileStatus::Success) {
        warn!("Document errors: {:?}", document.errors);
        return Status::Ok;
    }

    // Specific processing by document type.
    match file.type_ {
        FileType::RentReceipt => on_receipt_created(&db_pool, &file).await,
        _ => panic!(),
    }

    Status::Ok
}

// # Handlers

async fn on_receipt_created(db_pool: &DbPool, receipt: &Receipt) {
    let db = piteo::db(db_pool);

    let rent = db.rents().by_receipt_id(&receipt.id).unwrap();
    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    piteo::send_receipts(db_pool, input).await.ok();
}
