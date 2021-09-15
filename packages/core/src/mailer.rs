use async_trait::async_trait;
use eyre::Error;
use piteo_data::Email;
use serde::Serialize;
use std::fmt::Debug;

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

pub struct Mail {
    pub message_id: String,
}
