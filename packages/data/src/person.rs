use crate::common::Id;
use crate::schema::persons;
use crate::AccountId;
use crate::Address;
use crate::DateTime;
use crate::Email;
use crate::Name;
use crate::PhoneNumber;
use crate::Url;
use async_graphql::scalar;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;

// # Types

pub type PersonId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
#[graphql(name = "UserRole")]
pub enum PersonRole {
    Admin,
    Tenant,
    User,
    Viewer,
}

#[derive(Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct AuthId(String);

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Person {
    pub id: PersonId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub auth_id: AuthId,
    pub email: Email,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<Address>,
    pub photo_url: Option<Url>,
    pub role: Option<PersonRole>,
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "persons"]
pub struct PersonData {
    pub id: PersonId,
    pub account_id: Option<AccountId>,
    pub auth_id: Option<AuthId>,
    pub email: Option<Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<Address>,
}

// # Impls

impl AuthId {
    pub fn new(auth_id: String) -> Self {
        Self(auth_id)
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Default for AuthId {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Name for Person {
    fn first_name(&self) -> String {
        self.first_name.clone().unwrap_or_default()
    }

    fn last_name(&self) -> String {
        self.last_name.clone().unwrap_or_default()
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display_name())
    }
}

scalar!(AuthId, "AuthenticationID");

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Person {
        fn default() -> Self {
            Self {
                id: Default::default(),
                created_at: Default::default(),
                updated_at: Default::default(),
                phone_number: Default::default(),
                auth_id: Default::default(),
                email: Default::default(),
                first_name: Default::default(),
                last_name: Default::default(),
                address: Default::default(),
                photo_url: Default::default(),
                role: Default::default(),
                account_id: Default::default(),
            }
        }
    }

    #[test]
    fn test_display_name() {
        let person = Person {
            first_name: Some("John".to_string()),
            last_name: Some("DOE".to_string()),
            ..Default::default()
        };
        assert_eq!(person.display_name(), "John DOE");
    }
}
