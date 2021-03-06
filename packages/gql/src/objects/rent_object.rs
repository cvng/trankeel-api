use super::File;
use super::Lease;
use super::Payment;
use async_graphql::Context;
use async_graphql::Enum;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::Amount;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::LeaseId;
use trankeel::NoticeId;
use trankeel::ReceiptId;
use trankeel::RentId;

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
#[graphql(name = "RentStatus")]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

impl From<trankeel::RentStatus> for RentStatus {
    fn from(item: trankeel::RentStatus) -> Self {
        match item {
            trankeel::RentStatus::Open => Self::Pending,
            trankeel::RentStatus::Paid => Self::Settled,
            trankeel::RentStatus::PartiallyPaid => Self::Partial,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Rent {
    pub id: RentId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub period_end: DateTime,
    pub period_start: DateTime,
    pub amount: Amount,
    pub charges_amount: Option<Amount>,
    pub full_amount: Amount,
    pub status: RentStatus,
    pub lease_id: LeaseId,
    pub receipt_id: Option<ReceiptId>,
    pub notice_id: Option<NoticeId>,
    //
    pub delay: i64,
}

#[async_graphql::ComplexObject]
impl Rent {
    async fn lease(&self, ctx: &Context<'_>) -> Result<Lease> {
        Ok(ctx
            .data_unchecked::<Client>()
            .leases()
            .by_id(&self.lease_id)?
            .into())
    }

    #[graphql(name = "transactions")]
    async fn payments(&self, ctx: &Context<'_>) -> Result<Vec<Payment>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .payments()
            .by_rent_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn receipt(&self, ctx: &Context<'_>) -> Result<Option<File>> {
        Ok(self
            .receipt_id
            .map(|receipt_id| {
                ctx.data_unchecked::<Client>()
                    .files()
                    .by_id(&receipt_id)
                    .ok() // NotFound error is ok here.
            })
            .and_then(|receipt| receipt.map(Into::into)))
    }
}

impl From<trankeel::Rent> for Rent {
    fn from(item: trankeel::Rent) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            period_end: item.period_end,
            period_start: item.period_start,
            amount: item.amount,
            charges_amount: item.charges_amount,
            full_amount: item.full_amount,
            status: item.status.into(),
            lease_id: item.lease_id,
            receipt_id: item.receipt_id,
            notice_id: item.notice_id,
            delay: item.delay().num_days(),
        }
    }
}
