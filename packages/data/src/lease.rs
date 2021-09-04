use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseFurnishedData;
use async_graphql::Enum;
use chrono::Utc;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;

// # Types

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseType {
    Furnished,
    Naked,
}

#[derive(Clone, Queryable)]
pub struct Lease {
    pub account_id: Id,
    pub deposit_amount: Option<Amount>,
    pub effect_date: DateTime,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub r#type: LeaseType,
    pub lease_id: Option<Id>,
    pub property_id: Id,
    pub id: Id,
    pub data: Option<LeaseFurnishedData>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
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

impl FromSql<Text, Pg> for LeaseType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            Some(b"FURNISHED") => Ok(LeaseType::Furnished),
            Some(b"NAKED") => Ok(LeaseType::Naked),
            _ => Err("Unrecognized enum variant".into()),
        }
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
                r#type: LeaseType::Furnished,
                lease_id: Default::default(),
                property_id: Default::default(),
                id: Default::default(),
                data: Default::default(),
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
