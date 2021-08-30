use crate::database::Conn;
use crate::leases::LeaseData;
use crate::schema::lease;
use crate::schema::user;
use crate::Amount;
use crate::AuthId;
use chrono::DateTime;
use chrono::Utc;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::Text;
use eyre::Error;

pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Clone, FromSqlRow)]
pub enum LeaseType {
    Furnished,
    Naked,
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

// # Models

#[derive(Clone, Queryable)]
pub struct Lease {
    pub account_id: uuid::Uuid,
    pub deposit_amount: Option<Amount>,
    pub effect_date: chrono::NaiveDateTime,
    pub signature_date: Option<chrono::NaiveDateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub r#type: LeaseType,
    pub lease_id: Option<uuid::Uuid>,
    pub property_id: uuid::Uuid,
    pub id: uuid::Uuid,
    pub data: Option<LeaseData>,
    pub expired_at: Option<chrono::NaiveDateTime>,
    pub renew_date: Option<chrono::NaiveDateTime>,
}

impl Lease {
    pub fn rent_full_amount(&self) -> Amount {
        self.rent_amount + self.rent_charges_amount.unwrap_or_default()
    }

    pub fn status(&self) -> LeaseStatus {
        if self.expired_at.is_some()
            && Utc::now() > DateTime::<Utc>::from_utc(self.expired_at.unwrap(), Utc)
        {
            LeaseStatus::Ended
        } else {
            LeaseStatus::Active
        }
    }
}

// # Queries

pub fn load_by_auth_id(conn: &Conn, auth_id: &AuthId) -> Result<Vec<Lease>, Error> {
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
                effect_date: chrono::NaiveDateTime::from_timestamp(0, 0),
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
