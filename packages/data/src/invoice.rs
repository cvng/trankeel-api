use crate::id;
use crate::Amount;
use crate::DateTime;
use crate::PlanCode;
use fake::Fake;

id!(InvoiceId);

pub struct Invoice {
    pub id: InvoiceId,
    pub number: u32,
    pub amount_paid: Amount,
    pub invoice_pdf: String,
    pub period_end: DateTime,
    pub status: String,
    pub plan_code: PlanCode,
}
