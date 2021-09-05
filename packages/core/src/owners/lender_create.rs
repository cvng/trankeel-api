use crate::auth::UserInput;
use crate::companies::CompanyInput;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LenderIndividualInput {
    individual: UserInput,
}

#[derive(InputObject)]
pub struct LenderCompanyInput {
    company: CompanyInput,
}
