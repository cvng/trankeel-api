use chrono::NaiveDateTime;
use decimal::Decimal;
use eyre::Error;

/// Synthesis of collected rents.
pub struct Summary {
    pub since: NaiveDateTime,
    pub until: NaiveDateTime,
    //
    pub amount_expected: Decimal,
    pub amount_received: Decimal,
    pub amount_settled: Decimal,
    pub amount_partial: Decimal,
    pub amount_pending: Decimal,
    //
    pub n_expected: usize,
    pub n_received: usize,
    pub n_settled: usize,
    pub n_partial: usize,
    pub n_pending: usize,
    //
    pub ratio_expected: f64,
    pub ratio_received: f64,
    pub ratio_settled: f64,
    pub ratio_partial: f64,
    pub ratio_pending: f64,
    //
    pub variation_expected: f64,
    pub variation_received: f64,
    pub variation_settled: f64,
    pub variation_partial: f64,
    pub variation_pending: f64,
    //
    pub payment_rate: f64,
    pub occupation_rate: f64,
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            since: NaiveDateTime::from_timestamp(0, 0),
            until: NaiveDateTime::from_timestamp(0, 0),
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

pub fn get_summary() -> Result<Summary, Error> {
    Ok(Summary::default())
}
