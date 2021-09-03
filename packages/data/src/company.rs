use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum LegalEntityType {
    Eurl,
    Other,
    Sa,
    Sarl,
    Sas,
    Sasu,
    Sci,
    Scp,
    Snc,
}

impl From<String> for LegalEntityType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "EURL" => Self::Eurl,
            "OTHER" => Self::Other,
            "SA" => Self::Sa,
            "SARL" => Self::Sarl,
            "SAS" => Self::Sas,
            "SASU" => Self::Sasu,
            "SCI" => Self::Sci,
            "SCP" => Self::Scp,
            "SNC" => Self::Snc,
            _ => unimplemented!(),
        }
    }
}
