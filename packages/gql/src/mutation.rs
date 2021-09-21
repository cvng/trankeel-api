use crate::objects::Account;
use crate::objects::Error;
use crate::objects::File;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Task;
use crate::objects::Tenant;
use crate::objects::Transaction;
use crate::query::map_res;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo::auth::AccountActivatePlanInput;
use piteo::auth::AccountUpdateInput;
use piteo::auth::CreateUserWithAccountInput;
use piteo::files::FileInput;
use piteo::imports::ImportInput;
use piteo::leases::CreateFurnishedLeaseInput;
use piteo::leases::CreateReceiptsInput;
use piteo::leases::SendPaymentNoticeInput;
use piteo::leases::TransactionInput;
use piteo::leases::UpdateFurnishedLeaseInput;
use piteo::owners::UpdateIndividualLenderInput;
use piteo::properties::CreatePropertyInput;
use piteo::properties::UpdatePropertyInput;
use piteo::tenants::CreateTenantInput;
use piteo::tenants::UpdateTenantInput;
use piteo::AuthId;
use piteo::DbPool;
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

    async fn lease_furnished_create(
        &self,
        ctx: &Context<'_>,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::create_furnished_lease(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn lease_furnished_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::update_furnished_lease(db_pool.clone(), auth_id.clone(), input)?.into())
    }

    async fn lease_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::delete_lease(db_pool.clone(), auth_id.clone(), id.try_into()?)?.into())
    }

    async fn lender_individual_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(piteo::update_individual_lender(db_pool.clone(), auth_id.clone(), input)?.into())
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

    async fn rent_receipt_create(
        &self,
        ctx: &Context<'_>,
        input: CreateReceiptsInput,
    ) -> Result<CreateReceiptsPayload> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        match piteo::create_receipts(db_pool.clone(), auth_id.clone(), input).await {
            Ok(receipts) => Ok(CreateReceiptsPayload {
                receipts: Some(map_res(receipts)?),
                errors: None,
            }),
            Err(err) => Ok(CreateReceiptsPayload {
                receipts: None,
                errors: Some(vec![err.into()]),
            }),
        }
    }

    async fn send_payment_notice(
        &self,
        _input: SendPaymentNoticeInput,
    ) -> Result<SendPaymentNoticePayload> {
        Err(wip())
    }
}

// # Payloads

#[derive(async_graphql::SimpleObject)]
#[graphql(name = "RentReceiptPayload")]
struct CreateReceiptsPayload {
    errors: Option<Vec<Error>>,
    receipts: Option<Vec<File>>,
}

#[derive(async_graphql::SimpleObject)]
struct SendPaymentNoticePayload {
    errors: Option<Vec<Error>>,
    notices: Option<Vec<File>>,
}
