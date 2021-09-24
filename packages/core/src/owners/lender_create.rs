use crate::auth::CreatePersonInput;
use crate::companies::CompanyInput;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LenderIndividualInput {
    individual: CreatePersonInput,
}

#[derive(InputObject)]
pub struct LenderCompanyInput {
    company: CompanyInput,
}
