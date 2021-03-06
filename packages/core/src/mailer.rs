use crate::error::Error;
use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;
use trankeel_data::Email;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_data::Tenant;

#[async_trait]
pub trait Mailer {
    async fn batch(&self, mails: Vec<impl IntoMail + 'async_trait>) -> Result<Vec<Mail>, Error>;
}

pub trait IntoMail: Serialize + Clone + Debug + Send {
    fn template_id(&self) -> u32;
    fn subject(&self) -> String;
    fn recipients(&self) -> Vec<Contact>;
    fn data(&'static self) -> Box<dyn erased_serde::Serialize> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Contact {
    pub name: String,
    pub email: Email,
}

impl From<Person> for Contact {
    fn from(item: Person) -> Self {
        Self {
            name: item.display_name(),
            email: item.email,
        }
    }
}

impl From<Tenant> for Contact {
    fn from(item: Tenant) -> Self {
        Self {
            name: item.display_name(),
            email: item.email,
        }
    }
}

pub struct Mail {
    pub message_id: String,
}
