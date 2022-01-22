use crate::Candidacy;
use crate::Event;
use crate::File;
use crate::Id;
use crate::Lease;
use crate::Payment;
use crate::Rent;
use crate::Step;
use async_graphql::Union;

pub type EventableId = Id;

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
            Self::File(inner) => inner.id,
            Self::Rent(inner) => inner.id,
            Self::Step(inner) => inner.id,
            Self::Lease(inner) => inner.id,
            Self::Payment(inner) => inner.id,
            Self::Candidacy(inner) => inner.id,
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
