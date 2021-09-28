use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use diesel::sql_types::*;

// # Types

/// Synthesis of collected rents.
#[derive(Clone, Debug, QueryableByName)]
pub struct Summary {
    #[sql_type = "Uuid"]
    pub account_id: AccountId,
    #[sql_type = "Timestamptz"]
    pub current_month: DateTime,
    //
    #[sql_type = "Numeric"]
    pub amount_expected: Amount,
    #[sql_type = "Numeric"]
    pub amount_received: Amount,
    #[sql_type = "Numeric"]
    pub amount_settled: Amount,
    #[sql_type = "Numeric"]
    pub amount_partial: Amount,
    #[sql_type = "Numeric"]
    pub amount_pending: Amount,
    //
    #[sql_type = "Int4"]
    pub n_expected: i32,
    #[sql_type = "Int4"]
    pub n_received: i32,
    #[sql_type = "Int4"]
    pub n_settled: i32,
    #[sql_type = "Int4"]
    pub n_partial: i32,
    #[sql_type = "Int4"]
    pub n_pending: i32,
    //
    #[sql_type = "Float8"]
    pub ratio_expected: f64,
    #[sql_type = "Float8"]
    pub ratio_received: f64,
    #[sql_type = "Float8"]
    pub ratio_settled: f64,
    #[sql_type = "Float8"]
    pub ratio_partial: f64,
    #[sql_type = "Float8"]
    pub ratio_pending: f64,
    //
    #[sql_type = "Float8"]
    pub variation_expected: f64,
    #[sql_type = "Float8"]
    pub variation_received: f64,
    #[sql_type = "Float8"]
    pub variation_settled: f64,
    #[sql_type = "Float8"]
    pub variation_partial: f64,
    #[sql_type = "Float8"]
    pub variation_pending: f64,
    //
    #[sql_type = "Float8"]
    pub payment_rate: f64,
    #[sql_type = "Float8"]
    pub occupation_rate: f64,
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            current_month: DateTime::default(),
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
