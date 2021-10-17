use crate::templates::parse_template;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_kit::config::config;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CandidacyRejectedText {
    pub candidate_name: String,
}

impl CandidacyRejectedText {
    pub fn new(candidate: &Person) -> Self {
        Self {
            candidate_name: candidate.display_name(),
        }
    }

    pub fn as_text(&self) -> String {
        parse_template(
            &config()
                .templates("candidacy_rejected_text")
                .unwrap()
                .as_string()
                .unwrap(),
        )
        .unwrap()
        .render(&liquid::object!({ "candidate_name": self.candidate_name }))
        .unwrap()
    }
}

impl Display for CandidacyRejectedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidacy_rejected_text() {
        CandidacyRejectedText::default().to_string();
    }
}
