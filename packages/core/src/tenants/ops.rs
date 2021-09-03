use crate::database::Db;
use crate::schema::tenant;
use crate::AuthId;
use crate::DateTime;
use crate::Tenant;
use eyre::Error;
use serde::Deserialize;

// # Inputs

#[derive(Deserialize, Insertable)]
#[table_name = "tenant"]
pub struct TenantInput {
    pub apl: Option<bool>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub visale_id: Option<String>,
}

// # Operations

pub fn create_tenant<'a>(
    db: impl Db<'a>,
    auth_id: AuthId,
    data: TenantInput,
) -> Result<Tenant, Error> {
    db.tenants().insert(auth_id, data)
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tenant() {
        let tenant = create_tenant(
            crate::database::InMemoryDb,
            AuthId::default(),
            TenantInput {
                apl: Default::default(),
                birthdate: DateTime::from_timestamp(0, 0),
                birthplace: Default::default(),
                email: "tenant@piteo.dev".into(),
                first_name: Default::default(),
                last_name: Default::default(),
                note: Default::default(),
                phone_number: Default::default(),
                visale_id: Default::default(),
            },
        )
        .unwrap();
        assert_eq!(tenant.email, "tenant@piteo.dev");
    }
}
