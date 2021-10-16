use crate::error::Result;
use piteo_core::mailer::Contact;
use piteo_core::mailer::IntoMail;
use piteo_data::AsUrl;
use piteo_data::Candidacy;
use piteo_data::Person;
use piteo_data::Url;
use piteo_kit::config::config;
use piteo_kit::locale;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
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
