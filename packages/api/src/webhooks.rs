use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use trankeel::config;
use trankeel::event::DocumentGenerated;
use trankeel::providers::PdfmonkeyInput;
use trankeel::Client;

#[post("/webhooks/pdfmonkey", data = "<input>", format = "application/json")]
pub async fn pdfmonkey_request(client: &State<Client>, input: Json<PdfmonkeyInput>) -> Status {
    log::debug!("{}", config::log_json(&input.to_owned()));

    client
        .dispatch(vec![DocumentGenerated {
            document: input.document.clone(),
        }
        .into()])
        .await
        .map(|_| Status::Ok)
        .unwrap()
}
