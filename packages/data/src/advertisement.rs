use crate::schema::advertisements;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseId;
use crate::LeaseType;
use crate::LenderFlexibility;
use crate::PropertyId;

// # Types

pub type AdvertisementId = Id;

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Advertisement {
    pub id: AdvertisementId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub published: bool,
    pub lease_type: LeaseType,
    pub rent_amount: Amount,
    pub rent_charges_amount: Amount,
    pub deposit_amount: Amount,
    pub effect_date: DateTime,
    pub flexibility: LenderFlexibility,
    pub referral_lease_id: Option<LeaseId>,
    pub property_id: PropertyId,
}
