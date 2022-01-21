use crate::objects::Company;
use crate::objects::Person;
use crate::objects::Tenant;
use async_graphql::Interface;

#[derive(Interface)]
// #[graphql(field(name = "address", type = "Address"))]
#[graphql(field(name = "display_name", type = "String"))]
// #[graphql(field(name = "email", type = "Email"))]
// #[graphql(field(name = "phone_number", type = "PhoneNumber"))]
pub enum LegalIdentityInterface {
    Person(Person),
    Company(Company),
}

#[allow(clippy::large_enum_variant)]
#[derive(Interface)]
// #[graphql(field(name = "id", type = "ID"))]
#[graphql(field(name = "first_name", type = "String"))]
#[graphql(field(name = "last_name", type = "String"))]
#[graphql(field(name = "display_name", type = "String"))]
// #[graphql(field(name = "email", type = "Email"))]
// #[graphql(field(name = "phone_number", type = "PhoneNumber"))]
pub enum PersonInterface {
    Person(Person),
    Tenant(Tenant),
}
