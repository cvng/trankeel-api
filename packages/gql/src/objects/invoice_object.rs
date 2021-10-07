use piteo::Amount;
use piteo::DateTime;
use piteo::InvoiceId;
use piteo::PlanCode;

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
