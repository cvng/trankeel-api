use crate::rent_util;
use crate::schema::leases;
use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use crate::FileId;
use crate::FurnishedLeaseDetails;
use crate::FurnishedLeaseDuration;
use crate::Id;
use crate::PropertyId;
use crate::Rent;
use crate::RentId;
use crate::RentStatus;
use chrono::Utc;
use piteo_kit::locale;

pub type LeaseId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Leasetype"]
pub enum LeaseType {
    Furnished,
    Naked,
}

pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Lease {
    pub id: LeaseId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub deposit_amount: Option<Amount>,
    pub effect_date: DateTime,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub lease_id: Option<FileId>,
    pub property_id: PropertyId,
    pub details: Option<FurnishedLeaseDetails>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
    pub duration: FurnishedLeaseDuration,
}

#[derive(AsChangeset, Identifiable, Insertable)]
#[table_name = "leases"]
pub struct LeaseData {
    pub id: LeaseId,
    pub account_id: Option<AccountId>,
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
    pub duration: Option<FurnishedLeaseDuration>,
}

impl Lease {
    pub fn rent_full_amount(&self) -> Amount {
        self.rent_amount + self.rent_charges_amount.unwrap_or_default()
    }

    pub fn status(&self) -> LeaseStatus {
        match self.expired_at {
            Some(expired_at) if expired_at.inner() < Utc::now() => LeaseStatus::Ended,
            _ => LeaseStatus::Active,
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
            status: RentStatus::default(),
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
                type_: LeaseType::Furnished,
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
