use std::fs::File;
use std::io::Write;

const SCHEMA_PATH: &str = "schema.graphql";

/// Print the schema in SDL format. https://async-graphql.github.io/async-graphql/en/sdl_export.html
fn sdl_export(path: &str) {
    let schema_sdl = piteo_graphql::build_schema().unwrap().sdl();

    let mut file = File::create(path).unwrap();
    file.write_all(schema_sdl.as_bytes()).unwrap();

    println!("💫 GraphQL schema printed at {}", path);
}

fn main() {
    dotenv::dotenv().ok();

    sdl_export(SCHEMA_PATH);
}
