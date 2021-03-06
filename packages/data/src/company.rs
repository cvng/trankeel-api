use crate::id;
use crate::sql_schema::companies;
use crate::Address;
use crate::DateTime;
use crate::Email;
use crate::PhoneNumber;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Serialize;

// # Types

id!(CompanyId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Legalentitytype"]
pub enum LegalEntityType {
    Eurl,
    Other,
    Sa,
    Sarl,
    Sas,
    Sasu,
    Sci,
    Scp,
    Snc,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
#[table_name = "companies"]
pub struct Company {
    pub id: CompanyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub address: Option<Address>,
    pub email: Email,
    pub legal_entity: String,
    pub legal_entity_identifier: Option<String>,
    pub legal_entity_type: Option<LegalEntityType>,
    pub legal_entity_type_other: Option<String>,
    pub phone_number: Option<PhoneNumber>,
}

// # Impls

impl Company {
    pub fn display_name(&self) -> String {
        self.legal_entity.clone()
    }
}
