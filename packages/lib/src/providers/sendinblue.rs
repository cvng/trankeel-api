use crate::Provider;
use async_trait::async_trait;
use eyre::Error;
use piteo_core::mailer::IntoMail;
use piteo_core::mailer::Mail;
use piteo_core::mailer::Mailer;
use sendinblue::Mailer as Line;
use sendinblue::TransactionalBody;
use std::env;

const DEFAULT_SENDER_NAME: &str = "Piteo";

const DEFAULT_SENDER_EMAIL: &str = "support@piteo.fr";

pub struct Sendinblue(sendinblue::Sendinblue);

impl Provider for Sendinblue {
    fn init() -> Self {
        let api_key = env::var("SENDINBLUE_API_KEY").expect("STRIPE_SECRET_KEY");
        Self(sendinblue::Sendinblue::production(api_key))
    }
}

#[async_trait]
impl Mailer for Sendinblue {
    async fn batch(&self, mails: Vec<impl IntoMail + 'async_trait>) -> Result<Vec<Mail>, Error> {
        println!("Mailer.batch: {:?}", mails);

        let transactionals = mails.iter().map(to_transactional_body).collect::<Vec<_>>();
        let mut mails = vec![];

        for body in transactionals {
            let response = self.0.send_transactional_email(body).await?;

            mails.push(Mail {
                message_id: response.message_id,
            });
        }

        Ok(mails)
    }
}

fn to_transactional_body(mail: &impl IntoMail) -> TransactionalBody {
    let default_sender = Line::new(DEFAULT_SENDER_NAME, DEFAULT_SENDER_EMAIL);

    let mut body = TransactionalBody::builder()
        .set_sender(default_sender.clone())
        .reply_to(default_sender)
        .template_id(mail.template_id())
        .subject(mail.subject())
        .add_values(serde_json::to_value(mail).unwrap());

    for contact in mail.recipients() {
        body = body.add_to_mailer(Line::new(contact.name.clone(), contact.email.clone()));
    }

    body.create()
}
