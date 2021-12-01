use trankeel::DateTime;
use trankeel::ProfessionalWarrantId;

#[derive(SimpleObject)]
pub struct ProfessionalWarrant {
    pub id: ProfessionalWarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub name: String,
    pub identifier: String,
}

impl From<trankeel::ProfessionalWarrant> for ProfessionalWarrant {
    fn from(item: trankeel::ProfessionalWarrant) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            name: item.name,
            identifier: item.identifier,
        }
    }
}
