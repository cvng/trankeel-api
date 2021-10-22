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
use trankeel_data::PropertyId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct CreateAdvertisementInput {
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

// # Operation

pub fn create_advertisement(
    db: &impl Db,
    _auth_id: &AuthId,
    input: CreateAdvertisementInput,
) -> Result<Advertisement> {
    input.validate()?;

    db.advertisements().create(Advertisement {
        id: AdvertisementId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        published: input.published,
        lease_type: input.lease_type,
        rent_amount: input.rent_amount,
        rent_charges_amount: input.rent_charges_amount,
        deposit_amount: input.deposit_amount,
        effect_date: input.effect_date,
        flexibility: input.flexibility,
        referral_lease_id: input.referral_lease_id,
        property_id: input.property_id,
        title: input.title,
        description: input.description,
    })
}
