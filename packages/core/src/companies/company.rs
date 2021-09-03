use crate::database::Conn;
use crate::schema::company;
use crate::Address;
use crate::Id;
use crate::LegalEntity;
use diesel::prelude::*;
use eyre::Error;

// # Models

#[derive(Queryable)]
pub struct Company {
    pub address: Option<Address>,
    pub email: String,
    pub legal_entity: String,
    pub legal_entity_identifier: Option<String>,
    pub legal_entity_type: Option<String>,
    pub legal_entity_type_other: Option<String>,
    pub phone_number: Option<String>,
    pub id: Id,
}

impl Company {
    pub fn display_name(&self) -> String {
        self.legal_entity.clone()
    }
}

impl LegalEntity for Company {}

// # Queries

pub fn find(conn: &Conn, id: &Id) -> Result<Company, Error> {
    company::table
        .find(id)
        .first(conn)
        .map_err(|err| err.into())
}
