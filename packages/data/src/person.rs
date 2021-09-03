use async_graphql::Enum;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

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

#[derive(Clone, FromSqlRow)]
pub struct AuthId(String);

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
