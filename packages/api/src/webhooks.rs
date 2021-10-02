use piteo::Db;
use piteo::Document;
use piteo::FileData;
use piteo::FileStatus;
use piteo::FileType;
use piteo::PaymentNotice;
use piteo::Receipt;
use piteo::SendReceiptsInput;
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

    let mut client = piteo::init();
    let db = piteo::db(&client);

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
        FileType::RentReceipt => on_receipt_created(&mut client, &file).await,
        FileType::PaymentNotice => on_notice_created(&mut client, &file).await,
        _ => panic!(),
    }

    Status::Ok
}

// # Handlers

async fn on_receipt_created(client: &mut piteo::Client, receipt: &Receipt) {
    let db = piteo::db(client);

    let rent = db.rents().by_receipt_id(&receipt.id).unwrap();
    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    // Guess auth_id from given receipt (first user of the account).
    let lease = db.leases().by_receipt_id(&receipt.id).unwrap();
    let user = db
        .persons()
        .by_account_id(&lease.account_id)
        .map(|mut users| users.remove(0))
        .unwrap();
    let auth_id = user.auth_id.unwrap();
    client.set_auth_id(auth_id);

    piteo::send_receipts(client, input).await.ok();
}

async fn on_notice_created(client: &mut piteo::Client, notice: &PaymentNotice) {
    let db = piteo::db(client);

    let rent = db.rents().by_notice_id(&notice.id).unwrap();
    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    // Guess auth_id from given receipt (first user of the account).
    let lease = db.leases().by_notice_id(&notice.id).unwrap();
    let user = db
        .persons()
        .by_account_id(&lease.account_id)
        .map(|users| users.first().cloned())
        .unwrap()
        .unwrap();
    let auth_id = user.auth_id.unwrap();
    client.set_auth_id(auth_id);

    piteo::send_receipts(client, input).await.ok();
}
