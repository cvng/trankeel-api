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
pub type Server<T> = poem::Server<TcpListener<T>, Infallible>;

/// Build the web server.
///
/// https://github.com/poem-web/poem
pub fn server<T>(addr: T) -> Server<T>
where
    T: tokio::net::ToSocketAddrs + Send,
{
    Server::new(TcpListener::bind(addr)).name("trankeel")
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
