use crate::auth;
use crate::companies;
use crate::schema::lender;
use crate::schema::user;
use crate::AuthId;
use crate::Conn;
use diesel::prelude::*;
use eyre::Error;
use piteo_data::Lender;
use piteo_data::LenderId;
use piteo_data::LenderIdentity;

pub fn get_identity(conn: &Conn, id: LenderId) -> Result<LenderIdentity, Error> {
    let lender = lender_by_id(conn, id)?;

    match lender {
        Lender {
            individual_id: Some(individual_id),
            ..
        } => {
            let person = auth::person_by_id(conn, &individual_id)?;
            Ok(LenderIdentity::Individual(lender, person))
        }
        Lender {
            company_id: Some(company_id),
            ..
        } => {
            let company = companies::find(conn, &company_id)?;
            Ok(LenderIdentity::Company(lender, company))
        }
        _ => Err(Error::msg("Identity not found")),
    }
}

pub fn all_lenders(
    conn: &Conn,
    auth_id: &AuthId,
    id: Option<LenderId>,
) -> Result<Vec<Lender>, Error> {
    let auth_id = auth_id.clone();

    let query = lender::table
        .select(lender::all_columns)
        .left_join(user::table.on(user::account_id.eq(lender::account_id.nullable())))
        .filter(user::auth_id.eq(auth_id.inner()));

    if let Some(id) = id {
        return query
            .filter(lender::id.eq(id))
            .load(conn)
            .map_err(|err| err.into());
    }

    query.load(conn).map_err(|err| err.into())
}

pub fn lender_by_id(conn: &Conn, id: LenderId) -> Result<Lender, Error> {
    lender::table.find(id).first(conn).map_err(|err| err.into())
}
