#[derive(async_graphql::SimpleObject)]
pub struct Person {
    auth_id: Option<String>,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    photo_url: Option<String>,
    role: Option<String>,
    id: String,
    phone_number: Option<String>,
    account_id: Option<String>,
}

impl From<piteo_core::Person> for Person {
    fn from(item: piteo_core::Person) -> Self {
        Self {
            auth_id: item.auth_id,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            photo_url: item.photo_url,
            role: item.role,
            id: item.id.to_string(),
            phone_number: item.phone_number,
            account_id: item.account_id.map(|id| id.to_string()),
        }
    }
}
