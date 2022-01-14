use crate::error::Result;
use crate::mailer::IntoMail;
use crate::mailer::Mail;
use crate::mailer::Mailer;
use trankeel_data::Id;

#[derive(Clone)]
pub struct Sendinblue;

impl Sendinblue {
    pub fn init() -> Self {
        Self
    }
}

#[async_trait]
impl Mailer for Sendinblue {
    async fn batch(&self, mails: Vec<impl IntoMail + 'async_trait>) -> Result<Vec<Mail>> {
        let transactionals = mails.iter().collect::<Vec<_>>();
        let mut mails = vec![];

        for body in transactionals {
            write_message(body);

            mails.push(Mail {
                message_id: Id::new().to_string(),
            })
        }

        Ok(mails)
    }
}

fn write_message(mail: &impl IntoMail) {
    println!(
        "
        Subject: {:?}
        Recipients: {:?}
        Template: {:?}
        Data: {:?}
        {:-<80}
        ",
        mail.subject(),
        mail.recipients(),
        mail.template_id(),
        serde_json::to_string(mail).unwrap(),
        ""
    )
}
