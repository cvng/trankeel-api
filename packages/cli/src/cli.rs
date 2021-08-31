fn main() {
    dotenv::dotenv().ok();
    piteo_graphql::write_schema().ok();
}
