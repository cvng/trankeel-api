use crate::objects::Advertisement;
use crate::objects::Candidacy;
use crate::objects::Company;
use crate::objects::Discussion;
use crate::objects::File;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Lease;
use crate::objects::Message;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::ProfessionalWarrant;
use crate::objects::Rent;
use crate::objects::Step;
use crate::objects::Tenant;
use crate::objects::Warrant;

#[derive(Union)]
pub enum DiscussionItem {
    Candidacy(Candidacy),
}

impl From<trankeel::DiscussionItem> for DiscussionItem {
    fn from(item: trankeel::DiscussionItem) -> Self {
        match item {
            trankeel::DiscussionItem::Candidacy(inner) => Self::Candidacy(inner.into()),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum Eventable {
    Advertisement(Advertisement),
    Candidacy(Candidacy),
    Discussion(Discussion),
    File(File),
    Lease(Lease),
    Message(Message),
    Payment(Payment),
    Person(Person),
    Rent(Rent),
    Step(Step),
    Tenant(Tenant),
    Warrant(Warrant),
}

impl From<trankeel::Eventable> for Eventable {
    fn from(item: trankeel::Eventable) -> Self {
        match item {
            trankeel::Eventable::Advertisement(inner) => Self::Advertisement(inner.into()),
            trankeel::Eventable::Candidacy(inner) => Self::Candidacy(inner.into()),
            trankeel::Eventable::Discussion(inner) => Self::Discussion(inner.into()),
            trankeel::Eventable::File(inner) => Self::File(inner.into()),
            trankeel::Eventable::Lease(inner) => Self::Lease(inner.into()),
            trankeel::Eventable::Message(inner) => Self::Message(inner.into()),
            trankeel::Eventable::Payment(inner) => Self::Payment(inner.into()),
            trankeel::Eventable::Person(inner) => Self::Person(inner.into()),
            trankeel::Eventable::Rent(inner) => Self::Rent(inner.into()),
            trankeel::Eventable::Step(inner) => Self::Step(inner.into()),
            trankeel::Eventable::Tenant(inner) => Self::Tenant(inner.into()),
            trankeel::Eventable::Warrant(inner) => Self::Warrant(inner.into()),
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
