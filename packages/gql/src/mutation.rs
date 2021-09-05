use crate::inputs::AccountActivatePlanInput;
use crate::inputs::AccountUpdateInput;
use crate::inputs::CreateTenantInput;
use crate::inputs::CreateUserWithAccountInput;
use crate::inputs::FileInput;
use crate::inputs::ImportInput;
use crate::inputs::LeaseFurnishedInput;
use crate::inputs::LeaseFurnishedUpdateInput;
use crate::inputs::LenderIndividualUpdateInput;
use crate::inputs::PropertyInput;
use crate::inputs::PropertyUpdateInput;
use crate::inputs::RentReceiptInput;
use crate::inputs::TransactionInput;
use crate::inputs::UpdateTenantInput;
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
use piteo_core::AuthId;
use piteo_lib::DbPool;
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
        Ok(
            piteo_lib::create_user_with_account(db_pool.clone(), input.into())
                .await?
                .into(),
        )
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
        Ok(piteo_lib::create_tenant(db_pool.clone(), auth_id.clone(), input.into())?.into())
    }

    async fn tenant_update(&self, ctx: &Context<'_>, input: UpdateTenantInput) -> Result<Tenant> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo_lib::update_tenant(db_pool.clone(), auth_id.clone(), input.try_into()?)?.into())
    }

    async fn tenant_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo_lib::delete_tenant(db_pool.clone(), auth_id.clone(), id.try_into()?)?.into())
    }

    async fn property_create(&self, _input: PropertyInput) -> Result<Property> {
        Err(wip())
    }

    async fn property_update(&self, _input: PropertyUpdateInput) -> Result<Property> {
        Err(wip())
    }

    async fn property_delete(&self, _id: ID) -> Result<ID> {
        Err(wip())
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
