use crate::auth::UserInput;
use crate::companies::CompanyInput;

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualInput {
    individual: UserInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyInput {
    company: CompanyInput,
}
