use crate::database::Db;
use crate::AuthId;
use crate::DateTime;
use crate::Tenant;
use eyre::Error;
use piteo_data::PhoneNumber;
use validator::Validate;

// # Input

#[derive(Validate)]
pub struct CreateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub visale_id: Option<String>,
}

// # Operation

pub fn create_tenant(
    db: impl Db,
    auth_id: AuthId,
    input: CreateTenantInput,
) -> Result<Tenant, Error> {
    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;

    db.tenants().create(Tenant {
        account_id: account.id,
        apl: input.apl.unwrap_or_default(),
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email,
        first_name: input.first_name,
        last_name: input.last_name,
        note: input.note,
        phone_number: input.phone_number,
        visale_id: input.visale_id,
        auth_id: Default::default(),
        role: Default::default(),
        id: Default::default(),
        lease_id: Default::default(),
    })
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tenant() {
        let tenant = create_tenant(
            crate::testing::db(),
            AuthId::default(),
            CreateTenantInput {
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
