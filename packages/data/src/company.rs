use crate::Address;
use crate::Id;
use crate::LegalEntity;
use async_graphql::Enum;

// # Types

pub type CompanyId = Id;

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
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

#[derive(Queryable)]
pub struct Company {
    pub address: Option<Address>,
    pub email: String,
    pub legal_entity: String,
    pub legal_entity_identifier: Option<String>,
    pub legal_entity_type: Option<String>,
    pub legal_entity_type_other: Option<String>,
    pub phone_number: Option<String>,
    pub id: CompanyId,
}

// # Impls

impl Company {
    pub fn display_name(&self) -> String {
        self.legal_entity.clone()
    }
}

impl LegalEntity for Company {}

impl From<String> for LegalEntityType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "EURL" => Self::Eurl,
            "OTHER" => Self::Other,
            "SA" => Self::Sa,
            "SARL" => Self::Sarl,
            "SAS" => Self::Sas,
            "SASU" => Self::Sasu,
            "SCI" => Self::Sci,
            "SCP" => Self::Scp,
            "SNC" => Self::Snc,
            _ => unimplemented!(),
        }
    }
}
