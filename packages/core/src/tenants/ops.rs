use crate::database::Db;
use crate::AuthId;
use crate::DateTime;
use crate::Tenant;
use eyre::Error;
use piteo_data::Email;
use piteo_data::PhoneNumber;
use piteo_data::TenantData;

// # Inputs

pub struct TenantInput {
    pub apl: Option<bool>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub visale_id: Option<String>,
}

// # Operations

pub fn create_tenant(db: impl Db, auth_id: AuthId, input: TenantInput) -> Result<Tenant, Error> {
    let account = db.accounts().by_auth_id(auth_id)?;

    db.tenants().create(TenantData {
        account_id: account.id,
        apl: input.apl,
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email,
        first_name: input.first_name,
        last_name: input.last_name,
        note: input.note,
        phone_number: input.phone_number,
        visale_id: input.visale_id,
    })
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::InMemoryDb;

    #[test]
    fn create_tenant() {
        let tenant = super::create_tenant(
            InMemoryDb::new(),
            AuthId::default(),
            TenantInput {
                apl: Default::default(),
                birthdate: DateTime::default(),
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
