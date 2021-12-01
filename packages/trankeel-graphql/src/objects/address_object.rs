use trankeel::Inline;

#[derive(SimpleObject)]
pub struct Address {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
    //
    pub inline: String,
}

impl From<trankeel::Address> for Address {
    fn from(item: trankeel::Address) -> Self {
        Self {
            city: item.city.clone(),
            country: item.country.clone(),
            line1: item.line1.clone(),
            line2: item.line2.clone(),
            postal_code: item.postal_code.clone(),
            inline: item.inline(),
        }
    }
}
