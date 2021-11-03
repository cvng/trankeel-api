use crate::Advertisement;
use crate::Candidacy;
use crate::Discussion;
use crate::File;
use crate::Lease;
use crate::Message;
use crate::Payment;
use crate::Person;
use crate::Rent;
use crate::Step;
use crate::Tenant;
use crate::Warrant;

#[allow(clippy::large_enum_variant)]
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
