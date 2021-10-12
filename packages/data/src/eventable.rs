use crate::Candidacy;
use crate::Event;
use crate::File;
use crate::Id;
use crate::Payment;
use crate::Rent;

pub type EventableId = Id;

pub type EventableRow = (
    Event,
    Option<File>,
    Option<Rent>,
    Option<Payment>,
    Option<Candidacy>,
);

#[derive(Clone, Union)]
pub enum Eventable {
    File(File),
    Rent(Rent),
    Payment(Payment),
    Candidacy(Candidacy),
}

impl Eventable {
    pub fn id(&self) -> EventableId {
        match self {
            Self::File(inner) => inner.id,
            Self::Rent(inner) => inner.id,
            Self::Payment(inner) => inner.id,
            Self::Candidacy(inner) => inner.id,
        }
    }
}

impl From<EventableRow> for Eventable {
    fn from(item: EventableRow) -> Self {
        None.or_else(|| item.1.clone().map(Self::File))
            .or_else(|| item.2.clone().map(Self::Rent))
            .or_else(|| item.3.clone().map(Self::Payment))
            .or_else(|| item.4.clone().map(Self::Candidacy))
            .unwrap()
    }
}
