use crate::database::Conn;
use crate::leases::LeaseData;
use crate::schema::lease;
use crate::schema::user;
use crate::Amount;
use crate::AuthId;
use crate::DateTime;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::LeaseStatus;
use piteo_data::LeaseType;
use rust_chrono::Utc;

// # Models

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
    pub data: Option<LeaseData>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
}

impl Lease {
    pub fn rent_full_amount(&self) -> Amount {
        self.rent_amount + self.rent_charges_amount.unwrap_or_default()
    }

    pub fn status(&self) -> LeaseStatus {
        if self.expired_at.is_some()
            && Utc::now() > rust_chrono::DateTime::<Utc>::from_utc(self.expired_at.unwrap(), Utc)
        {
            LeaseStatus::Ended
        } else {
            LeaseStatus::Active
        }
    }
}

// # Queries

pub fn all_leases(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Lease>, Error> {
    lease::table
        .select(lease::all_columns)
        .left_join(user::table.on(user::accountId.eq(lease::accountId.nullable())))
        .filter(user::authId.eq(&auth_id.inner()))
        .load(conn)
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Lease {
        fn default() -> Self {
            Self {
                account_id: Default::default(),
                deposit_amount: Default::default(),
                effect_date: DateTime::from_timestamp(0, 0),
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
    fn rent_full_amount() {
        let lease = Lease {
            rent_amount: Amount::new(900, 0),
            rent_charges_amount: None,
            ..Default::default()
        };
        assert_eq!(lease.rent_full_amount().to_string(), "900".to_string());
    }
}
