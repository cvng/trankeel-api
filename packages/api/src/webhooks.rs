use poem::handler;
use poem::http::StatusCode;
use poem::web::Data;
use poem::web::Json;
use trankeel::config;
use trankeel::event::DocumentGenerated;
use trankeel::providers::PdfmonkeyInput;
use trankeel::Client;

#[handler]
pub async fn pdfmonkey_request(client: Data<&Client>, input: Json<PdfmonkeyInput>) -> StatusCode {
    log::debug!("{}", config::log_json(&input.0));

    client
        .dispatch(vec![DocumentGenerated {
            document: input.document.clone(),
        }
        .into()])
        .await
        .map(|_| StatusCode::OK)
        .unwrap()
}
