use crate::Address;
use crate::Id;
use crate::LegalEntity;
use crate::Name;
use async_graphql::Enum;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

// # Types

pub type PersonId = Id;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

#[derive(FromSqlRow, Clone, Serialize, Deserialize)]
pub struct AuthId(String);

#[derive(Queryable)]
pub struct Person {
    pub auth_id: AuthId,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<Address>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub id: PersonId,
    pub phone_number: Option<String>,
    pub account_id: Option<Id>,
}

// # Impls

impl From<String> for UserRole {
    fn from(item: String) -> Self {
        match item.as_str() {
            "ADMIN" => UserRole::Admin,
            "USER" => UserRole::User,
            "VIEWER" => UserRole::Viewer,
            _ => unimplemented!(),
        }
    }
}

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

impl FromSql<Text, Pg> for AuthId {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
        Ok(AuthId::new(value))
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

impl LegalEntity for Person {}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
