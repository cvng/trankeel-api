#[derive(SimpleObject)]
pub struct Error {
    message: String,
}

impl From<piteo::Error> for Error {
    fn from(item: piteo::Error) -> Self {
        Self {
            message: item.to_string(),
        }
    }
}
