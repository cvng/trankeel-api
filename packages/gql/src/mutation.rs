use crate::objects::Account;
use crate::objects::Advertisement;
use crate::objects::Candidacy;
use crate::objects::Error;
use crate::objects::File;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Task;
use crate::objects::Tenant;
use crate::query::map_res;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo::AcceptCandidacyInput;
use piteo::AccountActivatePlanInput;
use piteo::AccountUpdateInput;
use piteo::CreateAdvertisementInput;
use piteo::CreateCandidacyInput;
use piteo::CreateFileInput;
use piteo::CreateFurnishedLeaseInput;
use piteo::CreateNakedLeaseInput;
use piteo::CreateNoticesInput;
use piteo::CreatePropertyInput;
use piteo::CreateReceiptsInput;
use piteo::CreateTenantInput;
use piteo::CreateUserWithAccountInput;
use piteo::DeleteLeaseInput;
use piteo::DeletePropertyInput;
use piteo::DeleteTenantInput;
use piteo::ImportInput;
use piteo::TransactionInput;
use piteo::UpdateAdvertisementInput;
use piteo::UpdateFurnishedLeaseInput;
use piteo::UpdateIndividualLenderInput;
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
        Ok(piteo::create_user_with_account(&ctx.into(), input)
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
        Ok(piteo::create_tenant(&ctx.into(), input)?.into())
    }

    async fn tenant_update(&self, ctx: &Context<'_>, input: UpdateTenantInput) -> Result<Tenant> {
        Ok(piteo::update_tenant(&ctx.into(), input)?.into())
    }

    async fn tenant_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        Ok(piteo::delete_tenant(&ctx.into(), DeleteTenantInput { id: id.try_into()? })?.into())
    }

    async fn property_create(
        &self,
        ctx: &Context<'_>,
        input: CreatePropertyInput,
    ) -> Result<Property> {
        Ok(piteo::create_property(&ctx.into(), input)?.into())
    }

    async fn property_update(
        &self,
        ctx: &Context<'_>,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        Ok(piteo::update_property(&ctx.into(), input)?.into())
    }

    async fn property_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        Ok(piteo::delete_property(&ctx.into(), DeletePropertyInput { id: id.try_into()? })?.into())
    }

    async fn create_advertisement(
        &self,
        ctx: &Context<'_>,
        input: CreateAdvertisementInput,
    ) -> Result<Advertisement> {
        Ok(piteo::create_advertisement(&ctx.into(), input)?.into())
    }

    async fn update_advertisement(
        &self,
        ctx: &Context<'_>,
        input: UpdateAdvertisementInput,
    ) -> Result<Advertisement> {
        Ok(piteo::update_advertisement(&ctx.into(), input)?.into())
    }

    async fn lease_furnished_create(
        &self,
        ctx: &Context<'_>,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        Ok(piteo::create_furnished_lease(&ctx.into(), input)?.into())
    }

    async fn lease_furnished_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        Ok(piteo::update_furnished_lease(&ctx.into(), input)?.into())
    }

    async fn lease_naked_create(
        &self,
        _ctx: &Context<'_>,
        _input: CreateNakedLeaseInput,
    ) -> Result<Lease> {
        Err(wip())
    }

    async fn lease_delete(&self, ctx: &Context<'_>, id: ID) -> Result<ID> {
        Ok(piteo::delete_lease(&ctx.into(), DeleteLeaseInput { id: id.try_into()? })?.into())
    }

    async fn lender_individual_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        Ok(piteo::update_individual_lender(&ctx.into(), input)?.into())
    }

    async fn candidacy_create(
        &self,
        ctx: &Context<'_>,
        input: CreateCandidacyInput,
    ) -> Result<Candidacy> {
        Ok(piteo::create_candidacy(&ctx.into(), input)?.into())
    }

    async fn candidacy_accept(
        &self,
        ctx: &Context<'_>,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        Ok(piteo::accept_candidacy(&ctx.into(), input)?.into())
    }

    async fn transaction_create(&self, _input: TransactionInput) -> Result<Payment> {
        Err(wip())
    }

    async fn transaction_delete(&self, _id: ID) -> Result<ID> {
        Err(wip())
    }

    async fn file_upload(&self, _input: CreateFileInput) -> Result<File> {
        Err(wip())
    }

    async fn import_upload(&self, _input: ImportInput) -> Result<Task> {
        Err(wip())
    }

    #[graphql(name = "rentReceiptCreate")]
    async fn edit_rent_receipts(
        &self,
        ctx: &Context<'_>,
        input: CreateReceiptsInput,
    ) -> Result<CreateReceiptsPayload> {
        match piteo::create_receipts(&ctx.into(), input).await {
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

    #[graphql(name = "sendPaymentNotice")]
    async fn send_payment_notices(
        &self,
        ctx: &Context<'_>,
        input: CreateNoticesInput,
    ) -> Result<CreateNoticesPayload> {
        match piteo::create_notices(&ctx.into(), input).await {
            Ok(receipts) => Ok(CreateNoticesPayload {
                notices: Some(map_res(receipts)?),
                errors: None,
            }),
            Err(err) => Ok(CreateNoticesPayload {
                notices: None,
                errors: Some(vec![err.into()]),
            }),
        }
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
struct CreateNoticesPayload {
    errors: Option<Vec<Error>>,
    notices: Option<Vec<File>>,
}
