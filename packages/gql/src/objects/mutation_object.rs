use super::Account;
use super::Advertisement;
use super::Candidacy;
use super::File;
use super::Lease;
use super::Lender;
use super::Payment;
use super::Person;
use super::Property;
use super::Task;
use super::Tenant;
use crate::payloads::CreateNoticesPayload;
use crate::payloads::CreateReceiptsPayload;
use crate::payloads::DeleteDiscussionPayload;
use crate::payloads::PushMessagePayload;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AcceptCandidacyInput;
use piteo::AccountActivatePlanInput;
use piteo::AccountUpdateInput;
use piteo::AuthId;
use piteo::Client;
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
use piteo::DeleteDiscussionInput;
use piteo::DeleteLeaseInput;
use piteo::DeletePropertyInput;
use piteo::DeleteTenantInput;
use piteo::ImportInput;
use piteo::LeaseId;
use piteo::PaymentId;
use piteo::PropertyId;
use piteo::PushMessageInput;
use piteo::TenantId;
use piteo::TransactionInput;
use piteo::UpdateAdvertisementInput;
use piteo::UpdateFurnishedLeaseInput;
use piteo::UpdateIndividualLenderInput;
use piteo::UpdatePropertyInput;
use piteo::UpdateTenantInput;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn user_create_with_account(
        &self,
        ctx: &Context<'_>,
        input: CreateUserWithAccountInput,
    ) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_user_with_account(input)
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
        Ok(ctx
            .data_unchecked::<Client>()
            .create_tenant(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn tenant_update(&self, ctx: &Context<'_>, input: UpdateTenantInput) -> Result<Tenant> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_tenant(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn tenant_delete(&self, ctx: &Context<'_>, id: TenantId) -> Result<TenantId> {
        let input = DeleteTenantInput { id };
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_tenant(ctx.data::<AuthId>()?, input)?)
    }

    async fn property_create(
        &self,
        ctx: &Context<'_>,
        input: CreatePropertyInput,
    ) -> Result<Property> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_property(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn property_update(
        &self,
        ctx: &Context<'_>,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_property(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn property_delete(&self, ctx: &Context<'_>, id: PropertyId) -> Result<PropertyId> {
        let input = DeletePropertyInput { id };
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_property(ctx.data::<AuthId>()?, input)?)
    }

    async fn create_advertisement(
        &self,
        ctx: &Context<'_>,
        input: CreateAdvertisementInput,
    ) -> Result<Advertisement> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_advertisement(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn update_advertisement(
        &self,
        ctx: &Context<'_>,
        input: UpdateAdvertisementInput,
    ) -> Result<Advertisement> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_advertisement(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn lease_furnished_create(
        &self,
        ctx: &Context<'_>,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_furnished_lease(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn lease_furnished_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_furnished_lease(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn lease_naked_create(
        &self,
        _ctx: &Context<'_>,
        _input: CreateNakedLeaseInput,
    ) -> Result<Lease> {
        Err(wip())
    }

    async fn lease_delete(&self, ctx: &Context<'_>, id: LeaseId) -> Result<LeaseId> {
        let input = DeleteLeaseInput { id };
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_lease(ctx.data::<AuthId>()?, input)?)
    }

    async fn lender_individual_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_individual_lender(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn candidacy_create(
        &self,
        ctx: &Context<'_>,
        input: CreateCandidacyInput,
    ) -> Result<Candidacy> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_candidacy(input)?
            .into())
    }

    async fn candidacy_accept(
        &self,
        ctx: &Context<'_>,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        Ok(ctx
            .data_unchecked::<Client>()
            .accept_candidacy(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn transaction_create(&self, _input: TransactionInput) -> Result<Payment> {
        Err(wip())
    }

    async fn transaction_delete(&self, _id: PaymentId) -> Result<PaymentId> {
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
        Ok(ctx
            .data_unchecked::<Client>()
            .create_receipts(ctx.data::<AuthId>()?, input)
            .await
            .into())
    }

    #[graphql(name = "sendPaymentNotice")]
    async fn send_payment_notices(
        &self,
        ctx: &Context<'_>,
        input: CreateNoticesInput,
    ) -> Result<CreateNoticesPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_notices(ctx.data::<AuthId>()?, input)
            .await
            .into())
    }

    async fn push_message(
        &self,
        ctx: &Context<'_>,
        input: PushMessageInput,
    ) -> Result<PushMessagePayload> {
        Ok(ctx.data_unchecked::<Client>().push_message(input).into())
    }

    async fn delete_discussion(
        &self,
        ctx: &Context<'_>,
        input: DeleteDiscussionInput,
    ) -> Result<DeleteDiscussionPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_discussion(ctx.data::<AuthId>()?, input)
            .into())
    }
}
