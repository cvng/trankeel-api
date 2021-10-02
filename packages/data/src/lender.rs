use crate::common::Id;
use crate::schema::lenders;
use crate::AccountId;
use crate::CompanyId;
use crate::DateTime;
use crate::LegalIdentity;
use crate::PersonId;

// # Types

pub type LenderId = Id;

pub type LenderWithIdentity = (Lender, LegalIdentity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum LenderFlexibility {
    OneDay,
    ThreeDays,
    SevenDays,
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Lender {
    pub id: LenderId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "lenders"]
pub struct LenderData {
    pub id: LenderId,
    pub account_id: Option<AccountId>,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}
