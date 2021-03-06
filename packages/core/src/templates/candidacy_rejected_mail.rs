use crate::error::Result;
use crate::templates::parse_template;
use serde::Serialize;
use std::fmt;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_kit::config;

#[derive(Clone, Default, Debug, Serialize)]
pub struct CandidacyRejectedMail {
    pub candidate_name: String,
}

impl CandidacyRejectedMail {
    pub fn try_new(candidate: &Person) -> Result<Self> {
        Ok(Self {
            candidate_name: candidate.display_name(),
        })
    }

    pub fn as_text(&self) -> String {
        parse_template(
            &config::config()
                .templates("candidacy_rejected_mail")
                .unwrap()
                .as_string()
                .unwrap(),
        )
        .unwrap()
        .render(&liquid::object!({ "params": { "candidate_name": self.candidate_name } }))
        .unwrap()
    }
}

impl fmt::Display for CandidacyRejectedMail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidacy_rejected_mail() {
        CandidacyRejectedMail::default().to_string();
    }
}
