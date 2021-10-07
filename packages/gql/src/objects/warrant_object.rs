use crate::unions::WarrantIdentity;
use async_graphql::Context;
use async_graphql::Result;
use piteo::Client;
use piteo::DateTime;
use piteo::PersonId;
use piteo::ProfessionalWarrantId;
use piteo::TenantId;
use piteo::WarrantId;
use piteo::WarrantType;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Warrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub type_: WarrantType,
    pub tenant_id: TenantId,
    pub individual_id: Option<PersonId>,
    pub professional_id: Option<ProfessionalWarrantId>,
}

#[async_graphql::ComplexObject]
impl Warrant {
    async fn identity(&self, ctx: &Context<'_>) -> Result<WarrantIdentity> {
        Ok(ctx
            .data_unchecked::<Client>()
            .warrants()
            .by_id(&self.id)?
            .into())
    }
}

impl From<piteo::Warrant> for Warrant {
    fn from(item: piteo::Warrant) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            type_: item.type_,
            tenant_id: item.tenant_id,
            individual_id: item.individual_id,
            professional_id: item.professional_id,
        }
    }
}

impl From<piteo::WarrantWithIdentity> for Warrant {
    fn from(item: piteo::WarrantWithIdentity) -> Self {
        item.0.into()
    }
}
