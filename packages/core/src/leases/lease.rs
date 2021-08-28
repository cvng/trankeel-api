use crate::database::Conn;
use crate::leases::LeaseData;
use crate::schema::lease;
use crate::schema::user;
use crate::AuthId;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::Text;
use eyre::Error;

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
    pub deposit_amount: Option<decimal::Decimal>,
    pub effect_date: chrono::NaiveDateTime,
    pub signature_date: Option<chrono::NaiveDateTime>,
    pub rent_amount: decimal::Decimal,
    pub rent_charges_amount: Option<decimal::Decimal>,
    pub r#type: LeaseType,
    pub lease_id: Option<uuid::Uuid>,
    pub property_id: uuid::Uuid,
    pub id: uuid::Uuid,
    pub data: Option<LeaseData>,
    pub expired_at: Option<chrono::NaiveDateTime>,
    pub renew_date: Option<chrono::NaiveDateTime>,
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
