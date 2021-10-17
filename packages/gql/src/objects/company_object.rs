use super::Address;
use trankeel::CompanyId;
use trankeel::DateTime;
use trankeel::Email;
use trankeel::LegalEntityType;
use trankeel::PhoneNumber;

#[derive(SimpleObject)]
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
    //
    pub display_name: String,
}

impl From<trankeel::Company> for Company {
    fn from(item: trankeel::Company) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            address: item.address.clone().map(Into::into),
            email: item.email.clone(),
            legal_entity: item.legal_entity.clone(),
            legal_entity_identifier: item.legal_entity_identifier.clone(),
            legal_entity_type: item.legal_entity_type,
            legal_entity_type_other: item.legal_entity_type_other.clone(),
            phone_number: item.phone_number.clone(),
            display_name: item.display_name(),
        }
    }
}
