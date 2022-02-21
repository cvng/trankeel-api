use crate::id;
use crate::sql_schema::professional_warrants;
use crate::DateTime;
use fake::Fake;
use serde::Serialize;

id!(ProfessionalWarrantId);

#[derive(Clone, Debug, Serialize, Insertable, Queryable)]
pub struct ProfessionalWarrant {
    pub id: ProfessionalWarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub name: String,
    pub identifier: String,
}
