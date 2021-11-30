pub const DEFAULT_COUNTRY: &str = "FR";

pub const DEFAULT_CURRENCY: &str = "EUR";

pub const DEFAULT_OFFSET: i32 = 3600; // Without DST.

pub fn default_tz() -> chrono::FixedOffset {
    chrono::FixedOffset::east(DEFAULT_OFFSET)
}

pub fn text(key: &str) -> String {
    key.into()
}

pub fn currency(value: &str) -> String {
    format!("{} {}", value, DEFAULT_CURRENCY)
}
