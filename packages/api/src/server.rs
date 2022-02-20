use crate::guards::AuthMiddleware;
use crate::routes;
use crate::webhooks;
use poem::get;
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::post;
use poem::EndpointExt;
use poem::IntoEndpoint;
use poem::Route;
use std::convert::Infallible;
use trankeel::config::Config;
use trankeel::Result;
use trankeel_graphql::extensions::ApolloTracing;
use trankeel_graphql::extensions::Tracing;

/// Trankeel web server.
pub type Server<'a> = poem::Server<TcpListener<(&'a str, u16)>, Infallible>;

/// Build the web server.
///
/// https://github.com/poem-web/poem
pub fn server<'a>(config: Config) -> Server<'a> {
    Server::new(TcpListener::bind(("0.0.0.0", config.port))).name("trankeel")
}

/// Configure the web server.
///
/// https://github.com/poem-web/poem
pub async fn app(config: Config) -> Result<impl IntoEndpoint> {
    let client = trankeel::init(&config)?;

    let schema = trankeel_graphql::build_schema()
        .extension(ApolloTracing)
        .extension(Tracing)
        .data(client.clone())
        .finish();

    let app = Route::new()
        .at("/", get(routes::graphql_playground))
        .at("/graphql", post(routes::graphql_request))
        .at("/graphql/live", get(routes::graphql_live_request))
        .at("/webhooks/pdfmonkey", get(webhooks::pdfmonkey_request))
        .at("/debug/preview/<document_id>", get(routes::preview_request))
        .with(AuthMiddleware::new(config).await)
        .catch_all_error(routes::error_handler)
        .with(Cors::default())
        .data(client)
        .data(schema);

    Ok(app)
}
