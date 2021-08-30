use crate::common::LegalEntity;
use crate::database::Conn;
use crate::schema::lender;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use diesel::prelude::*;
use eyre::Error;
use std::fmt;
use std::fmt::Display;

// # Models

#[derive(Clone, Queryable)]
pub struct Lender {
    pub id: Id,
    pub account_id: Id,
    pub individual_id: Option<Id>,
    pub company_id: Option<Id>,
}

impl Lender {
    pub fn display_name(&self) -> String {
        "".to_string()
    }
}

impl LegalEntity for Lender {}

impl Display for Lender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display_name())
    }
}

// # Queries

pub fn all_lenders(conn: &Conn, auth_id: &AuthId, id: Option<Id>) -> Result<Vec<Lender>, Error> {
    let auth_id = auth_id.clone();

    let query = lender::table
        .select(lender::all_columns)
        .left_join(user::table.on(user::accountId.eq(lender::accountId.nullable())))
        .filter(user::authId.eq(auth_id.inner()));

    if let Some(id) = id {
        return query
            .filter(lender::id.eq(id))
            .load(conn)
            .map_err(|err| err.into());
    }

    query.load(conn).map_err(|err| err.into())
}
