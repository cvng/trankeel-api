use rocket::http::Status;
use rocket::info;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::warn;
use trankeel::templates::LeaseCreatedMail;
use trankeel::Client;
use trankeel::Document;
use trankeel::File;
use trankeel::FileStatus;
use trankeel::FileType;
use trankeel::LeaseFile;
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
        .update(&File {
            id: file.id,
            status: Some(document.status),
            download_url: document.download_url.clone(),
            preview_url: Some(document.preview_url.clone()),
            ..file
        })
        .unwrap();

    // If not success, stop processing.
    if file.status != Some(FileStatus::Success) {
        warn!("Document errors: {:?}", document.errors);
        return Status::Ok;
    }

    // Specific processing by document type.
    match file.type_ {
        FileType::LeaseDocument => handle_lease_created(&client, &file).await,
        FileType::RentReceipt => handle_receipt_or_notice_created(&client, &file).await,
        FileType::PaymentNotice => handle_receipt_or_notice_created(&client, &file).await,
    }

    Status::Ok
}

// # Handlers

async fn handle_lease_created(client: &Client, lease_file: &LeaseFile) {
    let lease = client.leases().by_lease_file_id(&lease_file.id).unwrap();
    let tenants = client.tenants().by_lease_id(&lease.id).unwrap();

    client
        .batch_mails(vec![
            LeaseCreatedMail::try_new(&lease, lease_file, tenants).unwrap()
        ])
        .await
        .unwrap();
}

async fn handle_receipt_or_notice_created(client: &Client, receipt_or_notice: &File) {
    let rent = match receipt_or_notice.type_ {
        FileType::RentReceipt => client.rents().by_receipt_id(&receipt_or_notice.id).unwrap(),
        FileType::PaymentNotice => client.rents().by_notice_id(&receipt_or_notice.id).unwrap(),
        _ => unimplemented!(),
    };

    let input = SendReceiptsInput {
        rent_ids: vec![rent.id],
    };

    client.send_receipts(input).await.unwrap();
}
