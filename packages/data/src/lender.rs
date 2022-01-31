use crate::id;
use crate::sql_schema::lenders;
use crate::AccountId;
use crate::CompanyId;
use crate::DateTime;
use crate::LegalIdentity;
use crate::PersonId;
use fake::Fake;

// # Types

id!(LenderId);

pub type LenderWithIdentity = (Lender, LegalIdentity);

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Lender {
    pub id: LenderId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}
