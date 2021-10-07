use crate::unions::LegalIdentity;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AccountId;
use piteo::Client;
use piteo::CompanyId;
use piteo::DateTime;
use piteo::LenderId;
use piteo::PersonId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Lender {
    pub id: LenderId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

#[async_graphql::ComplexObject]
impl Lender {
    async fn display_name(&self, ctx: &Context<'_>) -> Result<String> {
        Ok(ctx
            .data_unchecked::<Client>()
            .lenders()
            .by_id(&self.id)?
            .1
            .display_name())
    }

    async fn identity(&self, ctx: &Context<'_>) -> Result<LegalIdentity> {
        Ok(ctx
            .data_unchecked::<Client>()
            .lenders()
            .by_id(&self.id)?
            .into())
    }
}

impl From<piteo::Lender> for Lender {
    fn from(item: piteo::Lender) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            individual_id: item.individual_id,
            company_id: item.company_id,
        }
    }
}

impl From<piteo::LenderWithIdentity> for Lender {
    fn from(item: piteo::LenderWithIdentity) -> Self {
        item.0.into()
    }
}
