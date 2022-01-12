use crate::guards::AuthGuard;
use async_graphql_rocket::GraphQLRequest;
use async_graphql_rocket::GraphQLResponse;
use rocket::get;
use rocket::post;
use rocket::response::content::Html;
use rocket::State;
use trankeel_graphql::http::playground_source;
use trankeel_graphql::http::GraphQLPlaygroundConfig;
use trankeel_graphql::Schema;

#[get("/graphql")]
pub fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    request: GraphQLRequest,
    auth: AuthGuard,
) -> GraphQLResponse {
    let mut request = request;

    if let Some(auth_id) = auth.inner() {
        request = request.data(auth_id);
    }

    request.execute(schema).await
}

#[cfg(debug_assertions)]
pub mod dev_routes {
    use rocket::get;
    use rocket_dyn_templates::Template;
    use serde_json::Value;
    use trankeel::config;

    #[get("/preview/<document_id>")]
    pub fn preview_request(document_id: &str) -> Template {
        let mut data = config::read_json(format!("pdfmonkey/{}.json", document_id)).unwrap();

        let template_id = data.get("document_template_id").unwrap().as_str().unwrap();
        let template_name = config::template_by_id(template_id)
            .unwrap()
            .path
            .replace("templates/", "")
            .replace(".html.tera", "");

        let context: Value = data.get_mut("payload").unwrap().take();

        Template::render(template_name, &context)
    }
}
