use super::Feature;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::Amount;
use trankeel::PlanCode;
use trankeel::PlanId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Plan {
    pub id: PlanId,
    pub code: PlanCode,
    pub price: Option<Amount>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

#[async_graphql::ComplexObject]
impl Plan {
    async fn features(&self) -> Result<Vec<Feature>> {
        Ok(vec![])
    }
}
