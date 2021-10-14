use crate::objects::Candidacy;
use crate::objects::Company;
use crate::objects::File;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::ProfessionalWarrant;
use crate::objects::Rent;

#[derive(Union)]
pub enum DiscussionItem {
    Candidacy(Candidacy),
}

impl From<piteo::DiscussionItem> for DiscussionItem {
    fn from(item: piteo::DiscussionItem) -> Self {
        match item {
            piteo::DiscussionItem::Candidacy(inner) => Self::Candidacy(inner.into()),
        }
    }
}

#[derive(Union)]
pub enum Eventable {
    File(File),
    Rent(Rent),
    Payment(Payment),
    Candidacy(Candidacy),
}

impl From<piteo::Eventable> for Eventable {
    fn from(item: piteo::Eventable) -> Self {
        match item {
            piteo::Eventable::File(inner) => Self::File(inner.into()),
            piteo::Eventable::Rent(inner) => Self::Rent(inner.into()),
            piteo::Eventable::Payment(inner) => Self::Payment(inner.into()),
            piteo::Eventable::Candidacy(inner) => Self::Candidacy(inner.into()),
        }
    }
}

impl From<piteo::EventWithEventable> for Eventable {
    fn from(item: piteo::EventWithEventable) -> Self {
        item.1.into()
    }
}

#[derive(Union)]
pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

impl From<piteo::LeaseDetails> for LeaseDetails {
    fn from(item: piteo::LeaseDetails) -> Self {
        match item {
            piteo::LeaseDetails::FurnishedLeaseDetails(inner) => {
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

impl From<piteo::LegalIdentity> for LegalIdentity {
    fn from(item: piteo::LegalIdentity) -> Self {
        match item {
            piteo::LegalIdentity::Individual(inner) => Self::Individual(inner.into()),
            piteo::LegalIdentity::Company(inner) => Self::Company(inner.into()),
        }
    }
}

impl From<piteo::LenderWithIdentity> for LegalIdentity {
    fn from(item: piteo::LenderWithIdentity) -> Self {
        item.1.into()
    }
}

#[derive(Union)]
pub enum WarrantIdentity {
    Individual(Person),
    Professional(ProfessionalWarrant),
}

impl From<piteo::WarrantIdentity> for WarrantIdentity {
    fn from(item: piteo::WarrantIdentity) -> Self {
        match item {
            piteo::WarrantIdentity::Individual(inner) => Self::Individual(inner.into()),
            piteo::WarrantIdentity::Professional(inner) => Self::Professional(inner.into()),
        }
    }
}

impl From<piteo::WarrantWithIdentity> for WarrantIdentity {
    fn from(item: piteo::WarrantWithIdentity) -> Self {
        item.1.into()
    }
}
