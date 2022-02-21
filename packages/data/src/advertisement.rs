use crate::id;
use crate::sql_schema::advertisements;
use crate::Amount;
use crate::DateTime;
use crate::LeaseId;
use crate::LeaseType;
use crate::PropertyId;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Dummy;
use fake::Fake;
use serde::Serialize;

// # Types

id!(AdvertisementId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, DbEnum, Dummy, Enum)]
#[DieselType = "Entryflexibility"]
pub enum EntryFlexibility {
    OneDay,
    ThreeDays,
    SevenDays,
}

#[derive(Clone, Debug, Serialize, AsChangeset, Dummy, Identifiable, Insertable, Queryable)]
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
