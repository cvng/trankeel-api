use crate::FileId;

pub(crate) type Id = uuid::Uuid;

pub type ExternalId = String; // ID of an external service

pub trait Name {
    fn first_name(&self) -> String;

    fn last_name(&self) -> String;

    fn full_name(&self) -> String {
        self.display_name()
    }

    fn short_name(&self) -> String {
        self.display_name()
    }

    fn display_name(&self) -> String {
        [&self.first_name(), &self.last_name()]
            .iter()
            .map(|&v| v.clone())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string()
    }
}

pub trait Attachable {
    fn to_filename(&self, file_id: &FileId) -> String;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, DieselNewType)]
pub struct Email(String);

impl Email {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl From<String> for Email {
    fn from(item: String) -> Self {
        Self(item)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, DieselNewType)]
pub struct PhoneNumber(String);

impl From<String> for PhoneNumber {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<PhoneNumber> for String {
    fn from(item: PhoneNumber) -> Self {
        item.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, DieselNewType)]
pub struct Url(String);

impl From<String> for Url {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<Url> for String {
    fn from(item: Url) -> Self {
        item.0
    }
}

scalar!(Email);

scalar!(PhoneNumber);

scalar!(Url, "URL");
