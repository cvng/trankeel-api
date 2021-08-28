use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;

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
