use crate::id;
use crate::Candidacy;
use crate::CandidacyId;
use crate::Event;
use crate::File;
use crate::FileId;
use crate::Lease;
use crate::LeaseId;
use crate::Payment;
use crate::PaymentId;
use crate::Rent;
use crate::RentId;
use crate::Step;
use crate::StepId;
use async_graphql::Union;
use fake::Fake;

id!(EventableId);

impl From<FileId> for EventableId {
    fn from(item: FileId) -> Self {
        Self(item.0)
    }
}

impl From<RentId> for EventableId {
    fn from(item: RentId) -> Self {
        Self(item.0)
    }
}

impl From<StepId> for EventableId {
    fn from(item: StepId) -> Self {
        Self(item.0)
    }
}

impl From<LeaseId> for EventableId {
    fn from(item: LeaseId) -> Self {
        Self(item.0)
    }
}

impl From<PaymentId> for EventableId {
    fn from(item: PaymentId) -> Self {
        Self(item.0)
    }
}

impl From<CandidacyId> for EventableId {
    fn from(item: CandidacyId) -> Self {
        Self(item.0)
    }
}

pub type EventableRow = (
    Event,
    Option<File>,
    Option<Rent>,
    Option<Step>,
    Option<Lease>,
    Option<Payment>,
    Option<Candidacy>,
);

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Union)]
pub enum Eventable {
    File(File),
    Rent(Rent),
    Step(Step),
    Lease(Lease),
    Payment(Payment),
    Candidacy(Candidacy),
}

impl Eventable {
    pub fn id(&self) -> EventableId {
        match self {
            Self::File(inner) => inner.id.into(),
            Self::Rent(inner) => inner.id.into(),
            Self::Step(inner) => inner.id.into(),
            Self::Lease(inner) => inner.id.into(),
            Self::Payment(inner) => inner.id.into(),
            Self::Candidacy(inner) => inner.id.into(),
        }
    }
}

impl From<EventableRow> for Eventable {
    fn from(item: EventableRow) -> Self {
        None.or_else(|| item.1.clone().map(Self::File))
            .or_else(|| item.2.clone().map(Self::Rent))
            .or_else(|| item.3.clone().map(Self::Step))
            .or_else(|| item.4.clone().map(Self::Lease))
            .or_else(|| item.5.clone().map(Self::Payment))
            .or_else(|| item.6.clone().map(Self::Candidacy))
            .unwrap()
    }
}
