use crate::auth::CreatePersonInput;
use async_graphql::InputObject;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateIndividualLenderInput {
    pub individual: CreatePersonInput,
}
