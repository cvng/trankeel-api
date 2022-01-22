use async_graphql::SimpleObject;
use trankeel::Amount;
use trankeel::DateTime;
use trankeel::InvoiceId;
use trankeel::PlanCode;

#[derive(SimpleObject)]
pub struct Invoice {
    pub id: InvoiceId,
    pub number: u32,
    pub amount_paid: Amount,
    pub invoice_pdf: String,
    pub period_end: DateTime,
    pub status: String,
    pub plan_code: PlanCode,
}
