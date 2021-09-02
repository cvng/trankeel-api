use crate::common::LegalEntity;
use crate::database::Conn;
use crate::schema::user;
use crate::Address;
use crate::AuthId;
use crate::Id;
use crate::Name;
use diesel::dsl::FindBy;
use diesel::prelude::*;
use eyre::Error;
use std::fmt;
use std::fmt::Display;

// # Models

#[derive(Queryable)]
pub struct Person {
    pub auth_id: AuthId,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<Address>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub id: Id,
    pub phone_number: Option<String>,
    pub account_id: Option<Id>,
}

impl Name for Person {
    fn first_name(&self) -> String {
        self.first_name.clone().unwrap_or_default()
    }

    fn last_name(&self) -> String {
        self.last_name.clone().unwrap_or_default()
    }
}

impl LegalEntity for Person {}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display_name())
    }
}

// # Queries

pub fn find(conn: &Conn, auth_id: &AuthId) -> Result<Person, Error> {
    by_auth_id(auth_id).first(conn).map_err(|err| err.into())
}

pub fn person_by_id(conn: &Conn, id: &Id) -> Result<Person, Error> {
    user::table.find(id).first(conn).map_err(|err| err.into())
}

// # Utils

fn by_auth_id(auth_id: &AuthId) -> FindBy<user::table, user::authId, &str> {
    user::table.filter(user::authId.eq(auth_id.inner()))
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Person {
        fn default() -> Self {
            Self {
                auth_id: Default::default(),
                email: Default::default(),
                first_name: Default::default(),
                last_name: Default::default(),
                address: Default::default(),
                photo_url: Default::default(),
                role: Default::default(),
                id: Default::default(),
                phone_number: Default::default(),
                account_id: Default::default(),
            }
        }
    }

    #[test]
    fn display_name() {
        let person = Person {
            first_name: Some("John".to_string()),
            last_name: Some("DOE".to_string()),
            ..Default::default()
        };
        assert_eq!(person.display_name(), "John DOE");
    }
}
