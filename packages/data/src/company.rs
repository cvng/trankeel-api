use crate::Address;
use crate::DateTime;
use crate::Email;
use crate::Id;
use crate::PhoneNumber;

// # Types

pub type CompanyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
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

#[derive(Clone, Debug, Queryable)]
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
