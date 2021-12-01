use crate::error::Result;
use crate::mailer::Contact;
use crate::mailer::IntoMail;
use serde::Serialize;
use trankeel_data::Candidacy;
use trankeel_data::Person;
use trankeel_data::Url;
use trankeel_kit::config::config;
use trankeel_kit::locale;

#[derive(Clone, Default, Debug, Serialize)]
pub struct CandidacyCreatedMail {
    candidacy_url: Url,
    _recipients: Vec<Contact>,
}

impl CandidacyCreatedMail {
    pub fn try_new(candidacy: &Candidacy, candidate: &Person) -> Result<Self> {
        Ok(Self {
            candidacy_url: candidacy.as_url(),
            _recipients: vec![candidate.clone().into()],
        })
    }
}

impl IntoMail for CandidacyCreatedMail {
    fn template_id(&self) -> u32 {
        config()
            .templates("candidacy_created_mail")
            .unwrap()
            .id
            .parse::<u32>()
            .unwrap()
    }

    fn subject(&self) -> String {
        locale::text("candidacy_created_mail.subject")
    }

    fn recipients(&self) -> Vec<Contact> {
        self._recipients.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::parse_template;

    #[test]
    fn test_candicady_created_mail() {
        let mail = CandidacyCreatedMail::default();
        let text = config()
            .templates("candidacy_created_mail")
            .unwrap()
            .as_string()
            .unwrap();

        parse_template(&text)
            .unwrap()
            .render(&liquid::object!({
                "params": {
                    "candidacy_url": mail.candidacy_url,
                },
            }))
            .unwrap();
    }
}
