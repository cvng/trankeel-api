use crate::auth::UserUpdateInput;
use crate::companies::CompanyUpdateInput;
use async_graphql::InputObject;
use piteo_data::LenderId;

#[derive(InputObject)]
pub struct LenderIndividualUpdateInput {
    id: LenderId,
    individual: UserUpdateInput,
}

#[derive(InputObject)]
pub struct LenderCompanyUpdateInput {
    company: CompanyUpdateInput,
    id: LenderId,
}
