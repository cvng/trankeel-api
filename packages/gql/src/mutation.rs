use crate::objects::Account;
use crate::objects::File;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Task;
use crate::objects::Tenant;
use crate::objects::Transaction;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo::auth::AccountActivatePlanInput;
use piteo::auth::AccountUpdateInput;
use piteo::files::FileInput;
use piteo::imports::ImportInput;
use piteo::leases::LeaseFurnishedInput;
use piteo::leases::LeaseFurnishedUpdateInput;
use piteo::leases::RentReceiptInput;
use piteo::leases::TransactionInput;
use piteo::owners::LenderIndividualUpdateInput;
use piteo::AuthId;
use piteo::CreatePropertyInput;
use piteo::CreateTenantInput;
use piteo::CreateUserWithAccountInput;
use piteo::DbPool;
use piteo::UpdatePropertyInput;
use piteo::UpdateTenantInput;
use std::convert::TryInto;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn user_create_with_account(
        &self,
        ctx: &Context<'_>,
        input: CreateUserWithAccountInput,
    ) -> Result<Person> {
        let db_pool = ctx.data::<DbPool>()?;
        Ok(piteo::create_user_with_account(db_pool.clone(), input)
            .await?
            .into())
    }

    async fn account_update_payment_method(&self, _input: AccountUpdateInput) -> Result<Account> {
        Err(wip())
    }

    async fn account_activate_plan(&self, _input: AccountActivatePlanInput) -> Result<Account> {
        Err(wip())
    }

    async fn tenant_create(&self, ctx: &Context<'_>, input: CreateTenantInput) -> Result<Tenant> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::create_tenant(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn tenant_update(&self, ctx: &Context<'_>, input: UpdateTenantInput) -> Result<Tenant> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::update_tenant(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn tenant_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::delete_tenant(db_pool.clone(), auth_id.clone(), id.try_into()?)?.into())
    }

    async fn property_create(
        &self,
        ctx: &Context<'_>,
        input: CreatePropertyInput,
    ) -> Result<Property> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::create_property(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn property_update(
        &self,
        ctx: &Context<'_>,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::update_property(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn property_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::delete_property(db_pool.clone(), auth_id.clone(), id.try_into()?)?.into())
    }

    async fn lease_furnished_create(&self, _input: LeaseFurnishedInput) -> Result<Lease> {
        Err(wip())
    }

    async fn lease_delete(&self, _id: ID) -> Result<ID> {
        Err(wip())
    }

    async fn lease_furnished_update(&self, _input: LeaseFurnishedUpdateInput) -> Result<Lease> {
        Err(wip())
    }

    async fn lender_individual_update(
        &self,
        _input: LenderIndividualUpdateInput,
    ) -> Result<Lender> {
        Err(wip())
    }

    async fn transaction_create(&self, _input: TransactionInput) -> Result<Transaction> {
        Err(wip())
    }

    async fn transaction_delete(&self, _id: ID) -> Result<ID> {
        Err(wip())
    }

    async fn file_upload(&self, _input: FileInput) -> Result<File> {
        Err(wip())
    }

    async fn import_upload(&self, _input: ImportInput) -> Result<Task> {
        Err(wip())
    }

    async fn rent_receipt_create(&self, _input: RentReceiptInput) -> Result<RentReceiptPayload> {
        Err(wip())
    }
}

// # Payloads

#[derive(async_graphql::SimpleObject)]
struct RentReceiptPayload {
    receipt: File,
}
