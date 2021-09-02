use crate::auth;
use crate::common::LegalEntity;
use crate::companies;
use crate::companies::Company;
use crate::database::Conn;
use crate::schema::lender;
use crate::schema::user;
use crate::AuthId;
use crate::Id;
use crate::Name;
use crate::Person;
use diesel::prelude::*;
use eyre::Error;

pub enum Identity {
    Individual(Person),
    Company(Company),
}

impl Identity {
    pub fn display_name(&self) -> String {
        match self {
            Self::Individual(person) => person.display_name(),
            Self::Company(company) => company.display_name(),
        }
    }
}

// # Models

#[derive(Clone, Queryable)]
pub struct Lender {
    pub id: Id,
    pub account_id: Id,
    pub individual_id: Option<Id>,
    pub company_id: Option<Id>,
}

impl LegalEntity for Lender {}

// # Queries

pub fn get_identity(conn: &Conn, id: Id) -> Result<Identity, Error> {
    let lender = lender_by_id(conn, id)?;

    match lender {
        Lender {
            individual_id: Some(individual_id),
            ..
        } => {
            let individual = auth::person_by_id(conn, &individual_id)?;
            Ok(Identity::Individual(individual))
        }
        Lender {
            company_id: Some(company_id),
            ..
        } => {
            let company = companies::find(conn, &company_id)?;
            Ok(Identity::Company(company))
        }
        _ => Err(Error::msg("Identity not found")),
    }
}

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

pub fn lender_by_id(conn: &Conn, id: Id) -> Result<Lender, Error> {
    lender::table.find(id).first(conn).map_err(|err| err.into())
}
