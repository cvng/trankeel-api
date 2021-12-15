#[macro_use]
extern crate async_graphql;

mod auth;
mod candidacies;
mod client;
mod error;
mod files;
mod invites;
mod leases;
mod lenders;
mod messaging;
mod properties;
mod tenants;
mod warrants;
mod workflows;

pub use crate::client::init;
pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::error::Result;

pub use crate::auth::ActivateAccountPlanInput;
pub use crate::auth::AddressInput;
pub use crate::auth::CreateUserWithAccountInput;
pub use crate::auth::CreateUserWithAccountPayload;
pub use crate::auth::SignupUserFromInviteInput;
pub use crate::auth::SignupUserFromInvitePayload;
pub use crate::auth::UpdateAccountInput;
pub use crate::candidacies::AcceptCandidacyInput;
pub use crate::candidacies::CreateCandidacyInput;
pub use crate::files::CreateFileInput;
pub use crate::files::UploadImportInput;
pub use crate::leases::CreateFurnishedLeaseInput;
pub use crate::leases::CreateLeaseInput;
pub use crate::leases::CreateNakedLeaseInput;
pub use crate::leases::CreateNoticesInput;
pub use crate::leases::CreatePaymentInput;
pub use crate::leases::CreateReceiptsInput;
pub use crate::leases::DeleteLeaseInput;
pub use crate::leases::SendReceiptsInput;
pub use crate::leases::UpdateFurnishedLeaseInput;
pub use crate::lenders::UpdateIndividualLenderInput;
pub use crate::messaging::CreateDiscussionInput;
pub use crate::messaging::DeleteDiscussionInput;
pub use crate::messaging::PushMessageInput;
pub use crate::properties::CreateAdvertisementInput;
pub use crate::properties::CreatePropertyInput;
pub use crate::properties::DeletePropertyInput;
pub use crate::properties::UpdateAdvertisementInput;
pub use crate::properties::UpdatePropertyInput;
pub use crate::tenants::CreateTenantInput;
pub use crate::tenants::DeleteTenantInput;
pub use crate::tenants::UpdateTenantInput;
pub use crate::warrants::CreateProfessionalWarrantInput;
pub use crate::warrants::CreateWarrantInput;
pub use crate::workflows::CompleteStepInput;

pub use trankeel_core::handlers;
pub use trankeel_core::templates;
pub use trankeel_data::*;
