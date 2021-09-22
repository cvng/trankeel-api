use crate::objects::Person;

#[derive(async_graphql::Interface)]
#[graphql(name = "Person")]
#[graphql(field(name = "display_name", type = "String"))]
pub enum PersonInterface {
    Person(Person),
}
