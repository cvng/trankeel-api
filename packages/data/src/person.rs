use crate::id;
use crate::sql_schema::persons;
use crate::AccountId;
use crate::Address;
use crate::AuthId;
use crate::DateTime;
use crate::Email;
use crate::Name;
use crate::PhoneNumber;
use crate::Url;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Serialize;
use std::fmt;

// # Types

id!(PersonId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[graphql(name = "UserRole")]
#[DieselType = "Personrole"]
pub enum PersonRole {
    Admin,
    Candidate,
    Tenant,
    User,
    Viewer,
    Warrant,
}

impl Default for PersonRole {
    fn default() -> Self {
        Self::Viewer
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
pub struct Person {
    pub id: PersonId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub auth_id: Option<AuthId>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<Address>,
    pub photo_url: Option<Url>,
    pub role: PersonRole,
    pub phone_number: Option<PhoneNumber>,
}

// # Impls

impl Name for Person {
    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display_name())
    }
}

// # Tests

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Person {
        fn default() -> Self {
            Self {
                id: PersonId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                account_id: Default::default(),
                auth_id: Default::default(),
                email: Default::default(),
                first_name: Default::default(),
                last_name: Default::default(),
                address: Default::default(),
                photo_url: Default::default(),
                role: Default::default(),
                phone_number: Default::default(),
            }
        }
    }

    #[test]
    fn test_display_name() {
        let person = Person {
            first_name: "John".to_string(),
            last_name: "DOE".to_string(),
            ..Default::default()
        };
        assert_eq!(person.display_name(), "John DOE");
    }
}
