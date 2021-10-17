#[derive(SimpleObject)]
pub struct Error {
    message: String,
}

impl From<trankeel::Error> for Error {
    fn from(item: trankeel::Error) -> Self {
        Self {
            message: item.to_string(),
        }
    }
}
