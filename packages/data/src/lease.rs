use crate::common::Id;
use crate::schema::lease;
use crate::schema::leasetenant;
use crate::Amount;
use crate::DateTime;
use crate::FileId;
use crate::FurnishedLeaseDetails;
use crate::LenderId;
use crate::PropertyId;
use crate::TenantId;
use async_graphql::Enum;
use chrono::Utc;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub type LeaseId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseType {
    Furnished,
    Naked,
}

#[derive(Clone, Insertable, Queryable)]
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
            if Utc::now() > chrono::DateTime::<Utc>::from_utc(expired_at.inner(), Utc) {
                return LeaseStatus::Ended;
            }
        }
        LeaseStatus::Active
    }
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(lease.rent_full_amount().to_string(), "900".to_string());
    }
}
