use rocket::http::Status;
use rocket::info;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::warn;
use trankeel::Document;
use trankeel::FileData;
use trankeel::FileStatus;
use trankeel::FileType;
use trankeel::Notice;
use trankeel::Receipt;
use trankeel::SendReceiptsInput;

/// https://www.pdfmonkey.io/fr/doc/api/webhooks
#[derive(Debug, Deserialize)]
pub struct PdfmonkeyPayload {
    document: Document,
}

#[post("/webhooks/pdfmonkey", data = "<request>", format = "application/json")]
pub async fn pdfmonkey_request(request: Json<PdfmonkeyPayload>) -> Status {
    info!("Received pdfmonkey request: {:?}", request);

    let client = trankeel::init().unwrap();

    let document = request.document.clone();

    // General processing for any document.
    let file = client.files().by_external_id(&document.id).unwrap();
    let file = client
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
        FileType::RentReceipt => on_receipt_created(&client, &file).await,
        FileType::PaymentNotice => on_notice_created(&client, &file).await,
        _ => panic!(),
    }

    Status::Ok
}

// # Handlers

async fn on_receipt_created(client: &trankeel::Client, receipt: &Receipt) {
    let rent = client.rents().by_receipt_id(&receipt.id).unwrap();
    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    // Guess auth_id from given receipt (first user of the account).
    let lease = client.leases().by_receipt_id(&receipt.id).unwrap();
    let user = client
        .persons()
        .by_account_id(&lease.account_id)
        .map(|mut users| users.remove(0))
        .unwrap();
    let auth_id = user.auth_id.unwrap();

    client.send_receipts(&auth_id, input).await.ok();
}

async fn on_notice_created(client: &trankeel::Client, notice: &Notice) {
    let rent = client.rents().by_notice_id(&notice.id).unwrap();
    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    // Guess auth_id from given receipt (first user of the account).
    let lease = client.leases().by_notice_id(&notice.id).unwrap();
    let user = client
        .persons()
        .by_account_id(&lease.account_id)
        .unwrap()
        .first()
        .cloned()
        .unwrap();
    let auth_id = user.auth_id.unwrap();

    client.send_receipts(&auth_id, input).await.ok();
}
