use crate::objects::Candidacy;
use crate::objects::Company;
use crate::objects::File;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Lease;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::ProfessionalWarrant;
use crate::objects::Rent;
use crate::objects::Step;

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum Eventable {
    File(File),
    Rent(Rent),
    Step(Step),
    Lease(Lease),
    Payment(Payment),
    Candidacy(Candidacy),
}

impl From<trankeel::Eventable> for Eventable {
    fn from(item: trankeel::Eventable) -> Self {
        match item {
            trankeel::Eventable::File(inner) => Self::File(inner.into()),
            trankeel::Eventable::Rent(inner) => Self::Rent(inner.into()),
            trankeel::Eventable::Step(inner) => Self::Step(inner.into()),
            trankeel::Eventable::Lease(inner) => Self::Lease(inner.into()),
            trankeel::Eventable::Payment(inner) => Self::Payment(inner.into()),
            trankeel::Eventable::Candidacy(inner) => Self::Candidacy(inner.into()),
        }
    }
}

impl From<trankeel::EventWithEventable> for Eventable {
    fn from(item: trankeel::EventWithEventable) -> Self {
        item.1.into()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum DiscussionItem {
    Candidacy(Candidacy),
    Lease(Lease),
}

impl From<trankeel::DiscussionItem> for DiscussionItem {
    fn from(item: trankeel::DiscussionItem) -> Self {
        match item {
            trankeel::DiscussionItem::Candidacy(inner) => Self::Candidacy(inner.into()),
            trankeel::DiscussionItem::Lease(inner) => Self::Lease(inner.into()),
        }
    }
}

#[derive(Union)]
pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

impl From<trankeel::LeaseDetails> for LeaseDetails {
    fn from(item: trankeel::LeaseDetails) -> Self {
        match item {
            trankeel::LeaseDetails::FurnishedLeaseDetails(inner) => {
                Self::FurnishedLeaseDetails(inner.into())
            }
        }
    }
}

#[derive(Union)]
pub enum LegalIdentity {
    Individual(Person),
    Company(Company),
}

impl From<trankeel::LegalIdentity> for LegalIdentity {
    fn from(item: trankeel::LegalIdentity) -> Self {
        match item {
            trankeel::LegalIdentity::Individual(inner) => Self::Individual(inner.into()),
            trankeel::LegalIdentity::Company(inner) => Self::Company(inner.into()),
        }
    }
}

impl From<trankeel::LenderWithIdentity> for LegalIdentity {
    fn from(item: trankeel::LenderWithIdentity) -> Self {
        item.1.into()
    }
}

#[derive(Union)]
pub enum WarrantIdentity {
    Individual(Person),
    Professional(ProfessionalWarrant),
}

impl From<trankeel::WarrantIdentity> for WarrantIdentity {
    fn from(item: trankeel::WarrantIdentity) -> Self {
        match item {
            trankeel::WarrantIdentity::Individual(inner) => Self::Individual(inner.into()),
            trankeel::WarrantIdentity::Professional(inner) => Self::Professional(inner.into()),
        }
    }
}

impl From<trankeel::WarrantWithIdentity> for WarrantIdentity {
    fn from(item: trankeel::WarrantWithIdentity) -> Self {
        item.1.into()
    }
}
