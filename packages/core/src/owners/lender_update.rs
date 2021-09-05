use crate::auth::UserUpdateInput;
use crate::companies::CompanyUpdateInput;
use piteo_data::LenderId;

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualUpdateInput {
    id: LenderId,
    individual: UserUpdateInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyUpdateInput {
    company: CompanyUpdateInput,
    id: LenderId,
}
