use crate::error::Result;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::Advertisement;
use piteo_data::AdvertisementData;
use piteo_data::AdvertisementId;
use piteo_data::Amount;
use piteo_data::AuthId;
use piteo_data::DateTime;
use piteo_data::EntryFlexibility;
use piteo_data::LeaseId;
use piteo_data::LeaseType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct UpdateAdvertisementInput {
    pub id: AdvertisementId,
    pub published: Option<bool>,
    pub lease_type: Option<LeaseType>,
    pub rent_amount: Option<Amount>,
    pub rent_charges_amount: Option<Amount>,
    pub deposit_amount: Option<Amount>,
    pub effect_date: Option<DateTime>,
    pub flexibility: Option<EntryFlexibility>,
    pub referral_lease_id: Option<LeaseId>,
    pub title: Option<String>,
    pub description: Option<String>,
}

// # Operation

pub fn update_advertisement(
    db: &impl Db,
    _auth_id: &AuthId,
    input: UpdateAdvertisementInput,
) -> Result<Advertisement> {
    input.validate()?;

    db.advertisements().update(AdvertisementData {
        id: input.id,
        published: input.published,
        lease_type: input.lease_type,
        rent_amount: input.rent_amount,
        rent_charges_amount: input.rent_charges_amount,
        deposit_amount: input.deposit_amount,
        effect_date: input.effect_date,
        flexibility: input.flexibility,
        referral_lease_id: input.referral_lease_id,
        property_id: None,
        title: input.title,
        description: input.description,
    })
}
