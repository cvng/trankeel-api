use regex::Regex;

pub struct Author {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: String,
}

pub fn author(text: String) -> Result<Author, regex::Error> {
    let caps = Regex::new(r"(?P<first_name>\w+) (?P<last_name>\w+) <(?P<email>.*)>")?
        .captures(&text)
        .ok_or_else(|| regex::Error::Syntax("format: \"Dev PITEO <hello@piteo.dev>\"".into()))?;

    Ok(Author {
        first_name: caps["first_name"].into(),
        last_name: caps["last_name"].into(),
        email: caps["email"].into(),
    })
}
