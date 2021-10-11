use super::Discussion;
use super::Tenant;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AdvertisementId;
use piteo::CandidacyId;
use piteo::CandidacyStatus;
use piteo::Client;
use piteo::DateTime;
use piteo::TenantId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub advertisement_id: AdvertisementId,
    pub tenant_id: TenantId,
    pub move_in_date: DateTime,
    pub description: String,
}

#[async_graphql::ComplexObject]
impl Candidacy {
    async fn tenant(&self, ctx: &Context<'_>) -> Result<Tenant> {
        Ok(ctx
            .data_unchecked::<Client>()
            .tenants()
            .by_id(&self.tenant_id)?
            .into())
    }

    async fn discussion(&self, ctx: &Context<'_>) -> Result<Discussion> {
        Ok(ctx
            .data_unchecked::<Client>()
            .discussions()
            .by_candidacy_id(&self.id)?
            .into())
    }
}

impl From<piteo::Candidacy> for Candidacy {
    fn from(item: piteo::Candidacy) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            status: item.status,
            advertisement_id: item.advertisement_id,
            tenant_id: item.tenant_id,
            move_in_date: item.move_in_date,
            description: item.description,
        }
    }
}
