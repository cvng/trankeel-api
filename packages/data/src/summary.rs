use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use crate::Id;

pub type SummaryId = Id;

#[derive(Clone, Debug, Identifiable, Queryable, SimpleObject)]
#[table_name = "reports"]
pub struct Summary {
    pub id: SummaryId,
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

impl Default for Summary {
    fn default() -> Self {
        Self {
            id: Default::default(),
            account_id: Default::default(),
            created_at: DateTime::default(),
            amount_expected: Default::default(),
            amount_received: Default::default(),
            amount_settled: Default::default(),
            amount_partial: Default::default(),
            amount_pending: Default::default(),
            n_expected: Default::default(),
            n_received: Default::default(),
            n_settled: Default::default(),
            n_partial: Default::default(),
            n_pending: Default::default(),
            ratio_expected: Default::default(),
            ratio_received: Default::default(),
            ratio_settled: Default::default(),
            ratio_partial: Default::default(),
            ratio_pending: Default::default(),
            variation_expected: Default::default(),
            variation_received: Default::default(),
            variation_settled: Default::default(),
            variation_partial: Default::default(),
            variation_pending: Default::default(),
            payment_rate: Default::default(),
            occupation_rate: Default::default(),
        }
    }
}

table! {
    reports (id) {
        id -> Uuid,
        account_id -> Uuid,
        created_at -> Timestamptz,
        amount_expected -> Numeric,
        amount_received -> Numeric,
        amount_settled -> Numeric,
        amount_partial -> Numeric,
        amount_pending -> Numeric,
        n_expected -> Int4,
        n_received -> Int4,
        n_settled -> Int4,
        n_partial -> Int4,
        n_pending -> Int4,
        ratio_expected -> Float8,
        ratio_received -> Float8,
        ratio_settled -> Float8,
        ratio_partial -> Float8,
        ratio_pending -> Float8,
        variation_expected -> Float8,
        variation_received -> Float8,
        variation_settled -> Float8,
        variation_partial -> Float8,
        variation_pending -> Float8,
        payment_rate -> Float8,
        occupation_rate -> Float8,
    }
}
