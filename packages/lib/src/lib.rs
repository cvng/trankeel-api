#[macro_use]
extern crate async_graphql;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate validator;

mod auth;
mod billing;
mod candidacies;
mod client;
mod commands;
mod companies;
mod error;
mod files;
mod imports;
mod invites;
mod leases;
mod messaging;
mod owners;
mod properties;
mod reports;
mod templates;
mod tenants;
mod warrants;
mod workflows;

pub use crate::auth::AccountActivatePlanInput;
pub use crate::auth::AccountUpdateInput;
pub use crate::auth::AddressInput;
pub use crate::auth::CreateUserWithAccountInput;
pub use crate::auth::CreateUserWithAccountPayload;
pub use crate::auth::SignupUserFromInviteInput;
pub use crate::candidacies::AcceptCandidacyInput;
pub use crate::candidacies::CreateCandidacyInput;
pub use crate::client::init;
pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::files::CreateFileInput;
pub use crate::imports::ImportInput;
pub use crate::leases::AddExistingLeaseInput;
pub use crate::leases::AddExistingLeasePayload;
pub use crate::leases::CreateFurnishedLeaseInput;
pub use crate::leases::CreateNakedLeaseInput;
pub use crate::leases::CreateNoticesInput;
pub use crate::leases::CreateReceiptsInput;
pub use crate::leases::DeleteLeaseInput;
pub use crate::leases::SendReceiptsInput;
pub use crate::leases::TransactionInput;
pub use crate::leases::UpdateFurnishedLeaseInput;
pub use crate::messaging::CreateDiscussionInput;
pub use crate::messaging::DeleteDiscussionInput;
pub use crate::messaging::PushMessageInput;
pub use crate::messaging::PushMessagePayload;
pub use crate::owners::UpdateIndividualLenderInput;
pub use crate::properties::CreateAdvertisementInput;
pub use crate::properties::CreatePropertyInput;
pub use crate::properties::CreatePropertyPayload;
pub use crate::properties::DeletePropertyInput;
pub use crate::properties::UpdateAdvertisementInput;
pub use crate::properties::UpdatePropertyInput;
pub use crate::templates::LeaseCreatedMail;
pub use crate::tenants::CreateTenantInput;
pub use crate::tenants::CreateTenantPayload;
pub use crate::tenants::DeleteTenantInput;
pub use crate::tenants::UpdateTenantInput;
pub use crate::tenants::UpdateTenantPayload;
pub use crate::warrants::CreateProfessionalWarrantInput;
pub use crate::warrants::CreateWarrantInput;
pub use crate::workflows::CompleteStepInput;
pub use trankeel_data::*;

#[async_trait]
trait Command {
    type Input;
    type Payload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload>;
}
