use super::Lease;
use async_graphql::Context;
use async_graphql::Result;
use piteo::Amount;
use piteo::Client;
use piteo::DateTime;
use piteo::PaymentId;
use piteo::RentId;
use piteo::TransactionType;

#[derive(SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Transaction")]
pub struct Payment {
    pub id: PaymentId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub rent_id: RentId,
    pub amount: Amount,
    pub date: DateTime,
    pub type_: TransactionType,
    pub label: Option<String>,
}

#[async_graphql::ComplexObject]
impl Payment {
    async fn lease(&self, ctx: &Context<'_>) -> Result<Lease> {
        Ok(ctx
            .data_unchecked::<Client>()
            .leases()
            .by_rent_id(&self.rent_id)?
            .into())
    }
}

impl From<piteo::Payment> for Payment {
    fn from(item: piteo::Payment) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            rent_id: item.rent_id,
            amount: item.amount,
            date: item.date,
            type_: item.type_,
            label: item.label,
        }
    }
}
