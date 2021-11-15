use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::Advertisement;
use trankeel_data::AdvertisementId;
use trankeel_data::Amount;
use trankeel_data::AuthId;
use trankeel_data::DateTime;
use trankeel_data::EntryFlexibility;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
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

    let advertisement = db.advertisements().by_id(&input.id)?;

    db.advertisements().update(&Advertisement {
        id: input.id,
        published: input.published.unwrap_or(advertisement.published),
        lease_type: input.lease_type.unwrap_or(advertisement.lease_type),
        rent_amount: input.rent_amount.unwrap_or(advertisement.rent_amount),
        rent_charges_amount: input.rent_charges_amount,
        deposit_amount: input.deposit_amount.unwrap_or(advertisement.deposit_amount),
        effect_date: advertisement.effect_date,
        flexibility: input.flexibility,
        referral_lease_id: input.referral_lease_id,
        property_id: advertisement.property_id,
        title: input.title.unwrap_or(advertisement.title),
        description: input.description.unwrap_or(advertisement.description),
        ..advertisement
    })
}
