use crate::schema::professional_warrants;
use crate::DateTime;
use crate::Id;

pub type ProfessionalWarrantId = Id;

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct ProfessionalWarrant {
    pub id: ProfessionalWarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub name: String,
    pub identifier: String,
}
