use crate::id;
use crate::sql_schema::events;
use crate::DateTime;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Serialize;
use serde_json::Value;
use std::fmt;

id!(EventId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Eventtype"]
pub enum EventType {
    AccountCreated,
    AdvertisementCreated,
    AdvertisementUpdated,
    CandidacyAccepted,
    CandidacyCreated,
    CandidacyRejected,
    CompanyCreated,
    DiscussionCreated,
    DiscussionDeleted,
    DocumentGenerated,
    InviteAccepted,
    InviteCreated,
    LeaseAffected,
    LeaseCreated,
    LeaseDeleted,
    LeaseFileRequested,
    LeaseUpdated,
    LenderCreated,
    LenderUpdated,
    MessagePushed,
    NoticeCreated,
    PaymentCreated,
    PersonCreated,
    PropertyCreated,
    PropertyDeleted,
    PropertyUpdated,
    ReceiptCreated,
    ReceiptSent,
    StepCompleted,
    StepCreated,
    SubscriptionRequested,
    TenantCreated,
    TenantDeleted,
    TenantUpdated,
    WarrantCreated,
    WorkflowCreated,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_variant::to_variant_name(self).unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, Serialize, Insertable, Queryable)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub type_: EventType,
    pub payload: Value,
}
