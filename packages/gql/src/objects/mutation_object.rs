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
use crate::payloads::AddExistingLeasePayload;
use crate::payloads::CompleteStepPayload;
use crate::payloads::CreateNoticesPayload;
use crate::payloads::CreatePropertyPayload;
use crate::payloads::CreateReceiptsPayload;
use crate::payloads::CreateTenantPayload;
use crate::payloads::CreateUserWithAccountPayload;
use crate::payloads::DeleteDiscussionPayload;
use crate::payloads::PushMessagePayload;
use crate::payloads::UpdateTenantPayload;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AcceptCandidacyInput;
use trankeel::AccountActivatePlanInput;
use trankeel::AccountUpdateInput;
use trankeel::AddExistingLeaseInput;
use trankeel::AuthId;
use trankeel::Client;
use trankeel::CompleteStepInput;
use trankeel::CreateAdvertisementInput;
use trankeel::CreateCandidacyInput;
use trankeel::CreateFileInput;
use trankeel::CreateFurnishedLeaseInput;
use trankeel::CreateNakedLeaseInput;
use trankeel::CreateNoticesInput;
use trankeel::CreatePropertyInput;
use trankeel::CreateReceiptsInput;
use trankeel::CreateTenantInput;
use trankeel::CreateUserWithAccountInput;
use trankeel::DeleteDiscussionInput;
use trankeel::DeleteLeaseInput;
use trankeel::DeletePropertyInput;
use trankeel::DeleteTenantInput;
use trankeel::ImportInput;
use trankeel::LeaseId;
use trankeel::PaymentId;
use trankeel::PropertyId;
use trankeel::PushMessageInput;
use trankeel::SignupUserFromInviteInput;
use trankeel::TenantId;
use trankeel::TransactionInput;
use trankeel::UpdateAdvertisementInput;
use trankeel::UpdateFurnishedLeaseInput;
use trankeel::UpdateIndividualLenderInput;
use trankeel::UpdatePropertyInput;
use trankeel::UpdateTenantInput;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn user_create_with_account(
        &self,
        ctx: &Context<'_>,
        input: CreateUserWithAccountInput,
    ) -> Result<CreateUserWithAccountPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_user_with_account(input)
            .await?
            .into())
    }

    async fn signup_user_from_invite(
        &self,
        ctx: &Context<'_>,
        input: SignupUserFromInviteInput,
    ) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .signup_user_from_invite(input)
            .await?
            .into())
    }

    async fn account_update_payment_method(&self, _input: AccountUpdateInput) -> Result<Account> {
        Err(wip())
    }

    async fn account_activate_plan(&self, _input: AccountActivatePlanInput) -> Result<Account> {
        Err(wip())
    }

    async fn tenant_create(
        &self,
        ctx: &Context<'_>,
        input: CreateTenantInput,
    ) -> Result<CreateTenantPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_tenant(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn tenant_update(
        &self,
        ctx: &Context<'_>,
        input: UpdateTenantInput,
    ) -> Result<UpdateTenantPayload> {
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
    ) -> Result<CreatePropertyPayload> {
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

    async fn lease_add_existing(
        &self,
        ctx: &Context<'_>,
        input: AddExistingLeaseInput,
    ) -> Result<AddExistingLeasePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .add_existing_lease(ctx.data::<AuthId>()?, input)?
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
            .create_candidacy(input)
            .await?
            .into())
    }

    async fn candidacy_accept(
        &self,
        ctx: &Context<'_>,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        Ok(ctx
            .data_unchecked::<Client>()
            .accept_candidacy(ctx.data::<AuthId>()?, input)
            .await?
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
        Ok(ctx.data_unchecked::<Client>().push_message(input)?.into())
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

    async fn complete_step(
        &self,
        ctx: &Context<'_>,
        input: CompleteStepInput,
    ) -> Result<CompleteStepPayload> {
        Ok(ctx.data_unchecked::<Client>().complete_step(input).into())
    }
}
