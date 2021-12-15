use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use trankeel::handlers::DocumentGenerated;
use trankeel::Client;
use trankeel::Document;

/// https://www.pdfmonkey.io/fr/doc/api/webhooks
#[derive(Debug, Deserialize)]
pub struct PdfmonkeyPayload {
    pub document: Document,
}

#[post("/webhooks/pdfmonkey", data = "<request>", format = "application/json")]
pub async fn pdfmonkey_request(client: &State<Client>, request: Json<PdfmonkeyPayload>) -> Status {
    client
        .dispatch(vec![DocumentGenerated {
            document: request.document.clone(),
        }
        .into()])
        .await
        .map(|_| Status::Ok)
        .unwrap()
}
