use crate::common::Id;
use crate::schema::lease;
use crate::schema::leasetenant;
use crate::Amount;
use crate::DateTime;
use crate::FileId;
use crate::FurnishedLeaseDetails;
use crate::FurnishedLeaseDuration;
use crate::LenderId;
use crate::PropertyId;
use crate::Rent;
use crate::RentId;
use crate::RentStatus;
use crate::TenantId;
use async_graphql::Enum;
use chrono::Datelike;
use chrono::Utc;
use chronoutil::DateRule;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub type LeaseId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum LeaseType {
    Furnished,
    Naked,
}

#[derive(Clone, Debug, Insertable, Queryable)]
#[table_name = "lease"]
pub struct Lease {
    pub account_id: Id,
    pub deposit_amount: Option<Amount>,
    pub effect_date: DateTime,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub lease_id: Option<Id>,
    pub property_id: Id,
    pub id: Id,
    pub details: Option<FurnishedLeaseDetails>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
    pub duration: FurnishedLeaseDuration,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "lease"]
pub struct LeaseData {
    pub id: LeaseId,
    pub account_id: Option<Id>,
    pub deposit_amount: Option<Amount>,
    pub effect_date: Option<DateTime>,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Option<Amount>,
    pub rent_charges_amount: Option<Amount>,
    pub type_: Option<LeaseType>,
    pub lease_id: Option<FileId>,
    pub property_id: Option<PropertyId>,
    pub details: Option<FurnishedLeaseDetails>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
}

#[derive(Clone, Insertable, Queryable)]
#[table_name = "leasetenant"]
pub struct LeaseTenant {
    pub lease_id: LenderId,
    pub tenant_id: TenantId,
}

// # Impls

impl Lease {
    pub fn rent_full_amount(&self) -> Amount {
        self.rent_amount + self.rent_charges_amount.unwrap_or_default()
    }

    pub fn status(&self) -> LeaseStatus {
        if let Some(expired_at) = self.expired_at {
            if Utc::now() > expired_at.inner() {
                return LeaseStatus::Ended;
            }
        }
        LeaseStatus::Active
    }

    pub fn rents(&self) -> Vec<Rent> {
        let effect_date = self.effect_date;
        let duration_in_months = self.duration.in_months() as usize;

        let plain_rent = self.rent_amount;
        let plain_charges = self.rent_charges_amount.unwrap_or_default();
        let plain_total = self.rent_full_amount();

        DateRule::monthly(effect_date.inner().with_day(1).unwrap())
            .with_count(duration_in_months)
            .enumerate()
            .map(|(index, next_date)| {
                let end = DateTime::from(next_date).last_day_of_month();
                let start = next_date.into();

                let (start, end, rent, charges, total) = match index {
                    // First month.
                    0 => {
                        let period_days = end.duration_num_days(effect_date);
                        let month_days = effect_date.month_num_days();

                        let rent = prorata(plain_rent, period_days, month_days);
                        let charges = prorata(plain_charges, period_days, month_days);
                        let total = rent + charges;

                        (effect_date, end, rent, charges, total)
                    }
                    // Last month.
                    index if index + 1 == duration_in_months => {
                        let period_days = end.duration_num_days(start);
                        let month_days = effect_date.month_num_days();

                        let rent = prorata(plain_rent, period_days, month_days);
                        let charges = prorata(plain_charges, period_days, month_days);
                        let total = rent + charges;

                        (start, end, rent, charges, total)
                    }
                    // Plain month.
                    _ => (start, end, plain_rent, plain_charges, plain_total),
                };

                Rent {
                    id: RentId::new_v4(),
                    period_start: start,
                    period_end: end,
                    amount: rent,
                    charges_amount: Some(charges),
                    full_amount: total,
                    status: RentStatus::Pending,
                    lease_id: self.id,
                    receipt_id: None,
                    transaction_id: None,
                    notice_id: None,
                }
            })
            .collect()
    }
}

// # Utils

fn prorata(amount: Amount, actual: i64, expected: i64) -> Amount {
    let actual: rust_decimal::Decimal = actual.into();
    let expected: rust_decimal::Decimal = expected.into();
    (amount.inner() * actual / expected).into()
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;

    impl Default for Lease {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                deposit_amount: Default::default(),
                effect_date: DateTime::default(),
                signature_date: Default::default(),
                rent_amount: Default::default(),
                rent_charges_amount: Default::default(),
                type_: LeaseType::Furnished,
                lease_id: Default::default(),
                property_id: Default::default(),
                id: Default::default(),
                details: Default::default(),
                expired_at: Default::default(),
                renew_date: Default::default(),
                duration: Default::default(),
            }
        }
    }

    #[test]
    fn test_rent_full_amount() {
        let lease = Lease {
            rent_amount: Amount::new(900, 0),
            rent_charges_amount: None,
            ..Default::default()
        };
        assert_eq!(lease.rent_full_amount().inner(), Decimal::new(900, 0));
    }

    #[test]
    fn test_rents() {
        let lease = Lease {
            effect_date: Utc.ymd(2020, 1, 20).and_hms(0, 0, 0).into(),
            duration: FurnishedLeaseDuration::OneYear,
            rent_amount: Amount::new(900, 0),
            rent_charges_amount: Some(Amount::new(100, 0)),
            ..Default::default()
        };

        let rents = lease.rents();

        assert_eq!(rents.len(), 12);

        assert_eq!(
            rents[0].period_start.inner(),
            Utc.ymd(2020, 1, 20).and_hms_milli(0, 0, 0, 0)
        );
        assert_eq!(
            rents[0].period_end.inner(),
            Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            rents[0].full_amount.inner().round_dp(2),
            Decimal::new(38710, 2)
        );

        assert_eq!(
            rents[1].period_start.inner(),
            Utc.ymd(2020, 2, 1).and_hms_milli(0, 0, 0, 0)
        );
        assert_eq!(
            rents[1].period_end.inner(),
            Utc.ymd(2020, 2, 29).and_hms_milli(23, 59, 59, 999) // 2020 = leap year
        );
        assert_eq!(rents[1].full_amount.inner(), Decimal::new(1000, 0));

        assert_eq!(
            rents[2].period_start.inner(),
            Utc.ymd(2020, 3, 1).and_hms_milli(0, 0, 0, 0)
        );
        assert_eq!(
            rents[2].period_end.inner(),
            Utc.ymd(2020, 3, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(rents[2].full_amount.inner(), Decimal::new(1000, 0));

        assert_eq!(
            rents[11].period_start.inner(),
            Utc.ymd(2020, 12, 1).and_hms_milli(0, 0, 0, 0)
        );
        assert_eq!(
            rents[11].period_end.inner(),
            Utc.ymd(2020, 12, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(rents[11].full_amount.inner(), Decimal::new(1000, 0));
    }
}
