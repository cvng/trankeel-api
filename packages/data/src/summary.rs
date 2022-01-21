use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use async_graphql::SimpleObject;

pub type SummaryId = Id;

#[derive(Clone, Default, Debug, Identifiable, Queryable, SimpleObject)]
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
