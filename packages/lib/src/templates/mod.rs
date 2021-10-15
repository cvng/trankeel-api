mod receipt_document;
mod receipt_mail;

pub use receipt_document::*;
pub use receipt_mail::*;

#[allow(dead_code)]
fn parse_template(text: &str) -> Result<liquid::Template, liquid::Error> {
    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(text)
}
