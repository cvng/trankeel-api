use crate::FileId;

pub(crate) type Id = uuid::Uuid;

pub type ExternalId = String; // ID of an external service

pub type Email = String;

pub type PhoneNumber = String;

pub type Url = String;

pub trait LegalEntity {}

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
