mod account_object;
mod address_object;
mod advertisement_object;
mod candidacy_object;
mod company_object;
mod discussion_object;
mod error_object;
mod event_object;
mod feature_object;
mod file_object;
mod invoice_object;
mod lease_details_object;
mod lease_object;
mod lender_object;
mod message_object;
mod mutation_object;
mod payment_object;
mod person_object;
mod plan_object;
mod professional_warrant_object;
mod property_object;
mod query_object;
mod rent_object;
mod task_object;
mod tenant_object;
mod warrant_object;
mod workflow_object;

pub use account_object::*;
pub use address_object::*;
pub use advertisement_object::*;
pub use candidacy_object::*;
pub use company_object::*;
pub use discussion_object::*;
pub use error_object::*;
pub use event_object::*;
pub use feature_object::*;
pub use file_object::*;
pub use invoice_object::*;
pub use lease_details_object::*;
pub use lease_object::*;
pub use lender_object::*;
pub use message_object::*;
pub use mutation_object::*;
pub use payment_object::*;
pub use person_object::*;
pub use plan_object::*;
pub use professional_warrant_object::*;
pub use property_object::*;
pub use query_object::*;
pub use rent_object::*;
pub use task_object::*;
pub use tenant_object::*;
pub use warrant_object::*;
pub use workflow_object::*;

fn wip() -> async_graphql::Error {
    async_graphql::Error::new("wip!()")
}
