use crate::sql_schema::advertisements;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseId;
use crate::LeaseType;
use crate::PropertyId;

// # Types

pub type AdvertisementId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Entryflexibility"]
#[graphql(name = "LenderFlexibility")]
pub enum EntryFlexibility {
    OneDay,
    ThreeDays,
    SevenDays,
}

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Advertisement {
    pub id: AdvertisementId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub published: bool,
    pub lease_type: LeaseType,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub deposit_amount: Amount,
    pub effect_date: DateTime,
    pub flexibility: Option<EntryFlexibility>,
    pub referral_lease_id: Option<LeaseId>,
    pub property_id: PropertyId,
    pub title: String,
    pub description: String,
}
