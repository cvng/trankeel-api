use crate::guards::AuthIdGuard;
use async_graphql_poem::GraphQLProtocol;
use async_graphql_poem::GraphQLRequest;
use async_graphql_poem::GraphQLResponse;
use async_graphql_poem::GraphQLWebSocket;
use poem::handler;
use poem::web::websocket::WebSocket;
use poem::web::Data;
use poem::web::Html;
use poem::web::WithStatus;
use poem::Error;
use poem::IntoResponse;
use serde_json::Value;
use tera::Context;
use tera::Tera;
use trankeel::config;
use trankeel_graphql::http::playground_source;
use trankeel_graphql::http::GraphQLPlaygroundConfig;
use trankeel_graphql::http::ALL_WEBSOCKET_PROTOCOLS;
use trankeel_graphql::Schema;
use trankeel_graphql::ServerError;

pub async fn error_handler(err: Error) -> WithStatus<GraphQLResponse> {
    log::error!("{err}");

    GraphQLResponse(trankeel_graphql::Response::from_errors(vec![
        ServerError::new(err.to_string(), None),
    ]))
    .with_status(err.as_response().status())
}

#[handler]
pub fn graphql_playground() -> Html<String> {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/live"),
    ))
}

#[handler]
pub async fn graphql_request(
    request: GraphQLRequest,
    schema: Data<&Schema>,
    auth_id: Data<&AuthIdGuard>,
) -> GraphQLResponse {
    let mut request = request.0;

    if let Some(auth_id) = auth_id.inner() {
        request = request.data(auth_id);
    }

    schema.execute(request).await.into()
}

#[handler]
pub async fn graphql_live_request(
    websocket: WebSocket,
    protocol: GraphQLProtocol,
    schema: Data<&Schema>,
    auth_id: Data<&AuthIdGuard>,
) -> impl IntoResponse {
    let mut data = trankeel_graphql::Data::default();

    if let Some(auth_id) = auth_id.inner() {
        data.insert(auth_id);
    }

    let schema = schema.0.clone();

    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .with_data(data)
                .serve()
        })
}

#[handler]
pub fn preview_request(document_id: String) -> Html<String> {
    let mut data = config::read_json(format!("pdfmonkey/{}.json", document_id)).unwrap();

    let template_id = data.get("document_template_id").unwrap().as_str().unwrap();
    let template_name = config::template_by_id(template_id)
        .unwrap()
        .path
        .replace("templates/", "")
        .replace(".html.tera", "");

    let data: Value = data.get_mut("payload").unwrap().take();

    Html(render(&template_name, data))
}

fn render(template_name: &str, data: Value) -> String {
    Tera::new("templates/**/*") // TODO: use config
        .unwrap()
        .render(template_name, &Context::from_value(data).unwrap())
        .unwrap()
}
