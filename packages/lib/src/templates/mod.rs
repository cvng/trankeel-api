mod candidacy_accepted_mail;
mod candidacy_created_mail;
mod candidacy_rejected_text;
mod receipt_created_mail;
mod receipt_document;

pub use candidacy_accepted_mail::*;
pub use candidacy_created_mail::*;
pub use candidacy_rejected_text::*;
pub use receipt_created_mail::*;
pub use receipt_document::*;

#[allow(dead_code)]
pub(crate) fn parse_template(text: &str) -> Result<liquid::Template, liquid::Error> {
    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(text)
}
