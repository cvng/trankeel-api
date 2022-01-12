use colored_json::ToColoredJson;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use trankeel::config;
use trankeel::handlers::DocumentGenerated;
use trankeel::providers::PdfmonkeyInput;
use trankeel::Client;

#[post("/webhooks/pdfmonkey", data = "<input>", format = "application/json")]
pub async fn pdfmonkey_request(client: &State<Client>, input: Json<PdfmonkeyInput>) -> Status {
    config::log_json(&payload.to_owned());

    client
        .dispatch(vec![DocumentGenerated::with(&payload.document)])
        .await
        .map(|_| Status::Ok)
        .unwrap()
}
