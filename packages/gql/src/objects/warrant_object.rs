use crate::unions::WarrantIdentity;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::CandidacyId;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::PersonId;
use trankeel::ProfessionalWarrantId;
use trankeel::TenantId;
use trankeel::WarrantId;
use trankeel::WarrantType;

#[derive(SimpleObject)]
pub struct WarrantWithIdentity {
    pub warrant: Warrant,
    pub identity: WarrantIdentity,
}

impl From<trankeel::WarrantWithIdentity> for WarrantWithIdentity {
    fn from(item: trankeel::WarrantWithIdentity) -> Self {
        Self {
            warrant: item.warrant.into(),
            identity: item.identity.into(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Warrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub type_: WarrantType,
    pub tenant_id: Option<TenantId>,
    pub individual_id: Option<PersonId>,
    pub professional_id: Option<ProfessionalWarrantId>,
    pub candidacy_id: Option<CandidacyId>,
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

impl From<trankeel::Warrant> for Warrant {
    fn from(item: trankeel::Warrant) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            type_: item.type_,
            tenant_id: item.tenant_id,
            individual_id: item.individual_id,
            professional_id: item.professional_id,
            candidacy_id: item.candidacy_id,
        }
    }
}

impl From<trankeel::WarrantWithIdentity> for Warrant {
    fn from(item: trankeel::WarrantWithIdentity) -> Self {
        item.warrant.into()
    }
}
