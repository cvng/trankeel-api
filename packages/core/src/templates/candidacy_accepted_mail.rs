use crate::error::Result;
use crate::mailer::Contact;
use crate::mailer::IntoMail;
use serde::Serialize;
use trankeel_data::invite_url;
use trankeel_data::Candidacy;
use trankeel_data::Invite;
use trankeel_data::Person;
use trankeel_data::Url;
use trankeel_kit::config;
use trankeel_kit::locale;

#[derive(Clone, Default, Debug, Serialize)]
pub struct CandidacyAcceptedMail {
    invite_url: Url,
    _recipients: Vec<Contact>,
}

impl CandidacyAcceptedMail {
    pub fn try_new(_candidacy: &Candidacy, candidate: &Person, invite: &Invite) -> Result<Self> {
        Ok(Self {
            invite_url: invite_url(invite, candidate.email.clone()),
            _recipients: vec![candidate.clone().into()],
        })
    }
}

impl IntoMail for CandidacyAcceptedMail {
    fn template_id(&self) -> u32 {
        config::config()
            .templates("candidacy_accepted_mail")
            .unwrap()
            .id
            .parse::<u32>()
            .unwrap()
    }

    fn subject(&self) -> String {
        locale::text("candidacy_accepted_mail.subject")
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
    fn test_candidacy_accepted_mail() {
        let mail = CandidacyAcceptedMail::default();
        let text = config::config()
            .templates("candidacy_accepted_mail")
            .unwrap()
            .as_string()
            .unwrap();

        parse_template(&text)
            .unwrap()
            .render(&liquid::object!({
                "params": {
                    "invite_url": mail.invite_url,
                },
            }))
            .unwrap();
    }
}
