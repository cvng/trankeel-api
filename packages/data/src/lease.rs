use crate::id;
use crate::rent_util;
use crate::sql_schema::leases;
use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use crate::File;
use crate::FileId;
use crate::LeaseDetails;
use crate::LeaseDuration;
use crate::PropertyId;
use crate::Rent;
use crate::RentId;
use crate::RentStatus;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use chrono::Utc;
use diesel_derive_enum::DbEnum;
use fake::Dummy;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;
use trankeel_kit::locale;

id!(LeaseId);

pub type LeaseFile = File; // alias for a File

pub type LeaseFileId = FileId; // alias for a FileId

pub type LeaseWithRents = (Lease, Vec<Rent>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum LeaseStatus {
    Unsigned,
    Active,
    Expired,
}

impl Default for LeaseStatus {
    fn default() -> Self {
        Self::Unsigned
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Dummy, Enum)]
#[DieselType = "Leasetype"]
pub enum LeaseType {
    Furnished,
    Naked,
}

impl Default for LeaseType {
    fn default() -> Self {
        Self::Furnished
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
pub struct Lease {
    pub id: LeaseId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub deposit_amount: Amount,
    pub effect_date: DateTime,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub lease_id: Option<FileId>,
    pub property_id: PropertyId,
    pub details: Option<LeaseDetails>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
    pub duration: LeaseDuration,
}

impl Lease {
    pub fn rent_full_amount(&self) -> Amount {
        (self.rent_amount.inner() + self.rent_charges_amount.unwrap_or_default().inner()).into()
    }

    pub fn status(&self) -> LeaseStatus {
        match self {
            Self {
                expired_at: Some(expired_at),
                ..
            } if expired_at.inner() < Utc::now() => LeaseStatus::Expired,
            Self {
                signature_date: Some(_),
                ..
            } => LeaseStatus::Active,
            _ => LeaseStatus::default(),
        }
    }

    pub fn rents(&self) -> Vec<Rent> {
        rent_util::generate_rents(
            self.effect_date
                .inner()
                .with_timezone(&locale::default_tz()),
            self.duration.in_months() as usize,
            vec![
                self.rent_amount.inner(),
                self.rent_charges_amount.unwrap_or_default().inner(),
                self.rent_full_amount().inner(),
            ],
        )
        .into_iter()
        .map(|rent| Rent {
            id: RentId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            period_start: rent.0.into(),
            period_end: rent.1.into(),
            amount: rent.2[0].into(),
            charges_amount: Some(rent.2[1].into()),
            full_amount: rent.2[2].into(),
            status: if rent.0 < Utc::now() {
                RentStatus::Paid
            } else {
                RentStatus::Open
            },
            lease_id: self.id,
            receipt_id: None,
            notice_id: None,
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    impl Default for Lease {
        fn default() -> Self {
            Self {
                id: LeaseId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                account_id: Default::default(),
                deposit_amount: Default::default(),
                effect_date: DateTime::default(),
                signature_date: Default::default(),
                rent_amount: Default::default(),
                rent_charges_amount: Default::default(),
                type_: Default::default(),
                lease_id: Default::default(),
                property_id: Default::default(),
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
            rent_amount: Amount::new(90000),
            rent_charges_amount: None,
            ..Default::default()
        };
        assert_eq!(lease.rent_full_amount().inner(), dec!(900.00));
    }
}
