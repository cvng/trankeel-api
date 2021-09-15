use piteo::database::Db;
use piteo::leases::SendReceiptsInput;
use piteo::pdfmaker::Document;
use piteo::FileData;
use piteo::FileStatus;
use piteo::FileType;
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

    let db_pool = piteo::db_pool_from_env().unwrap();
    let db = piteo::db(db_pool.clone());

    let document = request.document.clone();

    let file = db.files().by_external_id(document.id.clone()).unwrap();
    let file = db
        .files()
        .update(&FileData {
            id: file.id,
            status: Some(document.status),
            download_url: document.download_url.clone(),
            preview_url: document.preview_url.clone(),
            ..Default::default()
        })
        .unwrap();

    if file.status != Some(FileStatus::Success) {
        warn!("Document errors: {:?}", document.errors);
        return Status::Ok; // If not success, stop processing.
    }

    match file.type_ {
        FileType::RentReceipt => {
            let rent = db.rents().by_receipt_id(file.id).unwrap();
            let input = SendReceiptsInput {
                rent_ids: vec![rent.id],
            };
            piteo::send_receipts(db_pool, input).await.unwrap();
            Status::Ok
        }
        _ => panic!(),
    }
}
