use super::Property;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::AdvertisementId;
use trankeel::Amount;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::EntryFlexibility;
use trankeel::LeaseId;
use trankeel::LeaseType;
use trankeel::PropertyId;

#[derive(SimpleObject)]
#[graphql(complex)]
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

#[async_graphql::ComplexObject]
impl Advertisement {
    async fn property(&self, ctx: &Context<'_>) -> Result<Property> {
        Ok(ctx
            .data_unchecked::<Client>()
            .properties()
            .by_advertisement_id(&self.id)?
            .into())
    }
}

impl From<trankeel::Advertisement> for Advertisement {
    fn from(item: trankeel::Advertisement) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            published: item.published,
            lease_type: item.lease_type,
            rent_amount: item.rent_amount,
            rent_charges_amount: item.rent_charges_amount,
            deposit_amount: item.deposit_amount,
            effect_date: item.effect_date,
            flexibility: item.flexibility,
            referral_lease_id: item.referral_lease_id,
            property_id: item.property_id,
            title: item.title,
            description: item.description,
        }
    }
}
