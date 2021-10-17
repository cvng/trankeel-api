use trankeel::AccountId;
use trankeel::Amount;
use trankeel::DateTime;

#[derive(SimpleObject)]
pub struct Summary {
    pub account_id: AccountId,
    pub created_at: DateTime,

    pub amount_expected: Amount,
    pub amount_received: Amount,
    pub amount_settled: Amount,
    pub amount_partial: Amount,
    pub amount_pending: Amount,

    pub n_expected: i32,
    pub n_received: i32,
    pub n_settled: i32,
    pub n_partial: i32,
    pub n_pending: i32,

    pub ratio_expected: f64,
    pub ratio_received: f64,
    pub ratio_settled: f64,
    pub ratio_partial: f64,
    pub ratio_pending: f64,

    pub variation_expected: f64,
    pub variation_received: f64,
    pub variation_settled: f64,
    pub variation_partial: f64,
    pub variation_pending: f64,

    pub payment_rate: f64,
    pub occupation_rate: f64,
}

impl From<trankeel::Summary> for Summary {
    fn from(item: trankeel::Summary) -> Self {
        Self {
            account_id: item.account_id,
            created_at: item.created_at,
            amount_expected: item.amount_expected,
            amount_received: item.amount_received,
            amount_settled: item.amount_settled,
            amount_partial: item.amount_partial,
            amount_pending: item.amount_pending,
            n_expected: item.n_expected,
            n_received: item.n_received,
            n_settled: item.n_settled,
            n_partial: item.n_partial,
            n_pending: item.n_pending,
            ratio_expected: item.ratio_expected,
            ratio_received: item.ratio_received,
            ratio_settled: item.ratio_settled,
            ratio_partial: item.ratio_partial,
            ratio_pending: item.ratio_pending,
            variation_expected: item.variation_expected,
            variation_received: item.variation_received,
            variation_settled: item.variation_settled,
            variation_partial: item.variation_partial,
            variation_pending: item.variation_pending,
            payment_rate: item.payment_rate,
            occupation_rate: item.occupation_rate,
        }
    }
}
