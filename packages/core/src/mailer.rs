use eyre::Error;
use piteo_data::Email;
use serde::Serialize;
use std::fmt;

pub trait Mailer {
    fn batch(&self, mails: Vec<impl IntoMail>) -> Result<Vec<Mail>, Error>;
}

pub trait IntoMail: Serialize + Clone + fmt::Debug {
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

pub struct Mail {
    pub message_id: String,
}
