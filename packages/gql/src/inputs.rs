#[derive(async_graphql::InputObject)]
pub struct UserWithAccountInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct AccountUpdateInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct AccountActivatePlanInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct TenantInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct TenantUpdateInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct PropertyInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct PropertyUpdateInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedUpdateInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualUpdateInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct TransactionInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct FileInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct ImportInput {
    id: String,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptInput {
    id: String,
}
