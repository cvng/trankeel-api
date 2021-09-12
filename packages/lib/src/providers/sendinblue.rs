use eyre::Error;
use piteo_core::mailer::IntoMail;
use piteo_core::mailer::Mail;
use piteo_core::mailer::Mailer;

pub struct Sendinblue;

impl Sendinblue {
    pub fn new() -> Self {
        Self
    }
}

impl Mailer for Sendinblue {
    fn batch(&self, mails: Vec<impl IntoMail>) -> Result<Vec<Mail>, Error> {
        println!("Mailer.batch: {:?}", mails);

        Ok(vec![Mail {
            message_id: Default::default(),
        }])
    }
}
