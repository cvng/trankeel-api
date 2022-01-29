use crate::payloads::AcceptCandidacyPayload;
use crate::payloads::CompleteStepPayload;
use crate::payloads::CreateAdvertisementPayload;
use crate::payloads::CreateCandidacyPayload;
use crate::payloads::CreateFurnishedLeasePayload;
use crate::payloads::CreateLeasePayload;
use crate::payloads::CreateNoticesPayload;
use crate::payloads::CreatePropertyPayload;
use crate::payloads::CreateReceiptsPayload;
use crate::payloads::CreateTenantPayload;
use crate::payloads::CreateUserWithAccountPayload;
use crate::payloads::DeleteDiscussionPayload;
use crate::payloads::DeleteLeasePayload;
use crate::payloads::DeletePropertyPayload;
use crate::payloads::DeleteTenantPayload;
use crate::payloads::PushMessagePayload;
use crate::payloads::SignupUserFromInvitePayload;
use crate::payloads::UpdateAdvertisementPayload;
use crate::payloads::UpdateFurnishedLeasePayload;
use crate::payloads::UpdateIndividualLenderPayload;
use crate::payloads::UpdatePropertyPayload;
use crate::payloads::UpdateTenantPayload;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AcceptCandidacyInput;
use trankeel::AuthId;
use trankeel::Client;
use trankeel::CompleteStepInput;
use trankeel::CreateAdvertisementInput;
use trankeel::CreateCandidacyInput;
use trankeel::CreateFurnishedLeaseInput;
use trankeel::CreateLeaseInput;
use trankeel::CreateNoticesInput;
use trankeel::CreatePropertyInput;
use trankeel::CreateReceiptsInput;
use trankeel::CreateTenantInput;
use trankeel::CreateUserWithAccountInput;
use trankeel::DeleteDiscussionInput;
use trankeel::DeleteLeaseInput;
use trankeel::DeletePropertyInput;
use trankeel::DeleteTenantInput;
use trankeel::PushMessageInput;
use trankeel::SignupUserFromInviteInput;
use trankeel::UpdateAdvertisementInput;
use trankeel::UpdateFurnishedLeaseInput;
use trankeel::UpdateIndividualLenderInput;
use trankeel::UpdatePropertyInput;
use trankeel::UpdateTenantInput;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_user_with_account(
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
    ) -> Result<SignupUserFromInvitePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .signup_user_from_invite(input)
            .await?
            .into())
    }

    async fn create_tenant(
        &self,
        ctx: &Context<'_>,
        input: CreateTenantInput,
    ) -> Result<CreateTenantPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_tenant(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn update_tenant(
        &self,
        ctx: &Context<'_>,
        input: UpdateTenantInput,
    ) -> Result<UpdateTenantPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_tenant(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn delete_tenant(
        &self,
        ctx: &Context<'_>,
        input: DeleteTenantInput,
    ) -> Result<DeleteTenantPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_tenant(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn create_property(
        &self,
        ctx: &Context<'_>,
        input: CreatePropertyInput,
    ) -> Result<CreatePropertyPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_property(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn update_property(
        &self,
        ctx: &Context<'_>,
        input: UpdatePropertyInput,
    ) -> Result<UpdatePropertyPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_property(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn delete_property(
        &self,
        ctx: &Context<'_>,
        input: DeletePropertyInput,
    ) -> Result<DeletePropertyPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_property(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn create_advertisement(
        &self,
        ctx: &Context<'_>,
        input: CreateAdvertisementInput,
    ) -> Result<CreateAdvertisementPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_advertisement(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn update_advertisement(
        &self,
        ctx: &Context<'_>,
        input: UpdateAdvertisementInput,
    ) -> Result<UpdateAdvertisementPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_advertisement(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn create_lease(
        &self,
        ctx: &Context<'_>,
        input: CreateLeaseInput,
    ) -> Result<CreateLeasePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_lease(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn create_furnished_lease(
        &self,
        ctx: &Context<'_>,
        input: CreateFurnishedLeaseInput,
    ) -> Result<CreateFurnishedLeasePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_furnished_lease(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn update_furnished_lease(
        &self,
        ctx: &Context<'_>,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<UpdateFurnishedLeasePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_furnished_lease(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn delete_lease(
        &self,
        ctx: &Context<'_>,
        input: DeleteLeaseInput,
    ) -> Result<DeleteLeasePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_lease(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn update_individual_lender(
        &self,
        ctx: &Context<'_>,
        input: UpdateIndividualLenderInput,
    ) -> Result<UpdateIndividualLenderPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .update_individual_lender(ctx.data::<AuthId>()?, input)?
            .into())
    }

    async fn create_candidacy(
        &self,
        ctx: &Context<'_>,
        input: CreateCandidacyInput,
    ) -> Result<CreateCandidacyPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_candidacy(input)
            .await?
            .into())
    }

    async fn accept_candidacy(
        &self,
        ctx: &Context<'_>,
        input: AcceptCandidacyInput,
    ) -> Result<AcceptCandidacyPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .accept_candidacy(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn create_receipts(
        &self,
        ctx: &Context<'_>,
        input: CreateReceiptsInput,
    ) -> Result<CreateReceiptsPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_receipts(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn create_notices(
        &self,
        ctx: &Context<'_>,
        input: CreateNoticesInput,
    ) -> Result<CreateNoticesPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .create_notices(ctx.data::<AuthId>()?, input)
            .await?
            .into())
    }

    async fn push_message(
        &self,
        ctx: &Context<'_>,
        input: PushMessageInput,
    ) -> Result<PushMessagePayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .push_message(input)
            .await?
            .into())
    }

    async fn delete_discussion(
        &self,
        ctx: &Context<'_>,
        input: DeleteDiscussionInput,
    ) -> Result<DeleteDiscussionPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .delete_discussion(input)
            .await?
            .into())
    }

    async fn complete_step(
        &self,
        ctx: &Context<'_>,
        input: CompleteStepInput,
    ) -> Result<CompleteStepPayload> {
        Ok(ctx
            .data_unchecked::<Client>()
            .complete_step(input)
            .await?
            .into())
    }
}
