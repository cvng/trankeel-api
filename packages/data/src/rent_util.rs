use chrono::DateTime;
use chrono::Datelike;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::Utc;
use chronoutil::DateRule;
use rust_decimal::Decimal;

pub type NaiveRent = (DateTime<Utc>, DateTime<Utc>, Vec<Decimal>);

/// Generates a series of rents from the given date.
pub fn generate_rents(
    from: DateTime<FixedOffset>,
    number: usize,
    values: Vec<Decimal>,
) -> Vec<NaiveRent> {
    DateRule::monthly(from)
        .with_count(number)
        .enumerate()
        .map(|(index, next_date)| {
            let start = next_date
                .with_day(1)
                .unwrap()
                .date()
                .and_hms_milli(0, 0, 0, 0);
            let end = next_date
                .with_day(month_num_days(next_date.year(), next_date.month()) as u32)
                .unwrap()
                .date()
                .and_hms_milli(23, 59, 59, 999);

            match index {
                // First month.
                0 => {
                    let expected = month_num_days(from.year(), from.month());
                    let actual = duration_num_days(&end.into(), &from.into());

                    let values = values
                        .iter()
                        .map(|value| prorata(value, expected, actual))
                        .collect();

                    (from.into(), end.into(), values)
                }

                // Last month.
                index if index + 1 == number => {
                    let expected = month_num_days(from.year(), from.month());
                    let actual = duration_num_days(&next_date.into(), &start.into());

                    let values = values
                        .iter()
                        .map(|value| prorata(value, expected, actual))
                        .collect();

                    (start.into(), next_date.into(), values)
                }

                // Plain month.
                _ => (start.into(), end.into(), values.clone()),
            }
        })
        .collect()
}

/// Returns the prorata of the given value.
fn prorata(value: &Decimal, expected: i64, actual: i64) -> Decimal {
    let expected: Decimal = expected.into();
    let actual: Decimal = actual.into();
    value * actual / expected
}

/// Returns the number of days in the given duration.
fn duration_num_days(dt: &DateTime<Utc>, rhs: &DateTime<Utc>) -> i64 {
    dt.signed_duration_since(*rhs).num_days() + 1
}

/// Returns the number of days in the given month.
fn month_num_days(year: i32, month: u32) -> i64 {
    let day = 1;
    NaiveDate::from_ymd(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        day,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, day))
    .num_days()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal_macros::dec;

    struct Rent {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        total: Decimal,
    }

    impl From<NaiveRent> for Rent {
        fn from(item: NaiveRent) -> Self {
            Self {
                start: item.0,
                end: item.1,
                total: item.2[0],
            }
        }
    }

    fn ymd(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> DateTime<Utc> {
        Utc.ymd(year, month, day)
            .and_hms_milli(hour, min, sec, milli)
    }

    #[test]
    fn test_generate_rents() {
        let rents = generate_rents(
            ymd(2020, 1, 20, 12, 0, 0, 0).into(),
            16,
            vec![dec!(1000.00)],
        )
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Rent>>();

        assert_eq!(rents.len(), 16);

        assert_eq!(rents[0].start, ymd(2020, 1, 20, 12, 0, 0, 0));
        assert_eq!(rents[0].end, ymd(2020, 1, 31, 23, 59, 59, 999));
        assert_eq!(rents[0].total.round_dp(2), dec!(387.10));

        assert_eq!(rents[1].start, ymd(2020, 2, 1, 0, 0, 0, 0));
        assert_eq!(rents[1].end, ymd(2020, 2, 29, 23, 59, 59, 999)); // 2020 = leap year
        assert_eq!(rents[1].total.round_dp(2), dec!(1000.00));

        assert_eq!(rents[2].start, ymd(2020, 3, 1, 0, 0, 0, 0));
        assert_eq!(rents[2].end, ymd(2020, 3, 31, 23, 59, 59, 999));
        assert_eq!(rents[2].total.round_dp(2), dec!(1000.00));

        assert_eq!(rents[15].start, ymd(2021, 4, 1, 0, 0, 0, 0));
        assert_eq!(rents[15].end, ymd(2021, 4, 20, 12, 0, 0, 0));
        assert_eq!(rents[15].total.round_dp(2), dec!(645.16));
    }
}
